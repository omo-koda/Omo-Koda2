defmodule OmokodaSwarm.Coordinator do
  @moduledoc """
  Advanced Agent Coordination for the Omokoda Swarm.
  Ports the strategies from Swibe (hierarchical, democratic, competitive, pipeline).
  """

  use GenServer
  require Logger

  defstruct [:name, :coordination, :agents, :rounds, :message_log]

  @doc """
  Starts the coordinator.
  """
  def start_link(name, coordination \\ :hierarchical) do
    GenServer.start_link(__MODULE__, {name, coordination}, name: name)
  end

  @doc """
  Dispatches a task to the team using the configured strategy.
  """
  def coordinate(coordinator, task) do
    GenServer.call(coordinator, {:coordinate, task}, 60_000)
  end

  @doc """
  Adds an agent to the team.
  """
  def add_agent(coordinator, role, config \\ %{}) do
    GenServer.call(coordinator, {:add_agent, role, config})
  end

  # GenServer callbacks

  @impl true
  def init({name, coordination}) do
    state = %__MODULE__{
      name: name,
      coordination: coordination,
      agents: %{},
      rounds: [],
      message_log: []
    }
    {:ok, state}
  end

  @impl true
  def handle_call({:add_agent, role, config}, _from, state) do
    # Start the agent process via SwarmSupervisor
    agent_id = "#{state.name}_#{role}"
    case OmokodaSwarm.SwarmSupervisor.start_agent(agent_id, config) do
      {:ok, pid} ->
        new_agents = Map.put(state.agents, role, %{id: agent_id, pid: pid, role: role, weight: config[:weight] || 1.0})
        {:reply, {:ok, agent_id}, %{state | agents: new_agents}}
      {:error, reason} ->
        {:reply, {:error, reason}, state}
    end
  end

  @impl true
  def handle_call({:coordinate, task}, _from, state) do
    agents_list = Map.values(state.agents)
    
    if Enum.empty?(agents_list) do
      {:reply, {:error, :no_agents}, state}
    else
      result = execute_strategy(state.coordination, state, task, agents_list)
      new_rounds = state.rounds ++ [%{task: task, strategy: state.coordination, result: result, timestamp: System.system_time(:millisecond)}]
      {:reply, {:ok, result}, %{state | rounds: new_rounds}}
    end
  end

  # Strategy Implementations

  defp execute_strategy(:hierarchical, _state, task, agents) do
    [lead | workers] = agents
    Logger.info("[COORDINATOR] Hierarchical strategy: #{lead.role} leading for task: #{task}")
    
    # 1. Lead plans
    {:ok, plan} = OmokodaSwarm.Agent.delegate_task(lead.id, "Plan task: #{task}")
    
    # 2. Delegate to workers
    worker_results = Enum.map(workers, fn worker ->
      OmokodaSwarm.Agent.delegate_task(worker.id, "Execute subtask from plan for: #{task}")
    end)
    
    # 3. Lead synthesizes
    {:ok, synthesis} = OmokodaSwarm.Agent.delegate_task(lead.id, "Synthesize results: #{inspect(worker_results)}")
    
    %{synthesis: synthesis, worker_results: worker_results}
  end

  defp execute_strategy(:democratic, _state, task, agents) do
    Logger.info("[COORDINATOR] Democratic strategy for task: #{task}")
    
    # 1. Agents propose solutions
    solutions = Enum.map(agents, fn agent ->
      {:ok, solution} = OmokodaSwarm.Agent.delegate_task(agent.id, "Propose solution for: #{task}")
      %{agent: agent, solution: solution}
    end)
    
    # 2. Vote (simplified weighted consensus)
    winner = Enum.max_by(solutions, fn s -> s.agent.weight end)
    
    %{winner: winner.solution, solutions: solutions}
  end

  defp execute_strategy(:competitive, _state, task, agents) do
    Logger.info("[COORDINATOR] Competitive strategy for task: #{task}")
    
    # Agents race (simplified: first in list for now, in real BEAM would be first to finish)
    results = Enum.map(agents, fn agent ->
      {:ok, result} = OmokodaSwarm.Agent.delegate_task(agent.id, "Compete to solve: #{task}")
      %{agent: agent, result: result}
    end)
    
    %{winner: hd(results).result, leaderboard: results}
  end

  defp execute_strategy(:pipeline, _state, task, agents) do
    Logger.info("[COORDINATOR] Pipeline strategy for task: #{task}")
    
    final_output = Enum.reduce(agents, task, fn agent, acc ->
      {:ok, output} = OmokodaSwarm.Agent.delegate_task(agent.id, "Transform input: #{inspect(acc)}")
      output
    end)
    
    %{output: final_output}
  end
end
