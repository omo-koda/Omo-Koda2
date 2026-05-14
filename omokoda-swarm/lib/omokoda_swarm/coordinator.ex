defmodule OmokodaSwarm.Coordinator do
  @moduledoc """
  Coordinates operations across the swarm.
  """

  use GenServer

  defstruct [:active_tasks, :agent_states]

  @doc """
  Starts the coordinator.
  """
  def start_link(opts \\ []) do
    GenServer.start_link(__MODULE__, opts, name: __MODULE__)
  end

  @doc """
  Submits a task to the swarm for processing.
  """
  def submit_task(task, options \\ []) do
    GenServer.call(__MODULE__, {:submit_task, task, options})
  end

  @doc """
  Gets the current swarm status.
  """
  def get_status do
    GenServer.call(__MODULE__, :get_status)
  end

  @doc """
  Scales the swarm by adding/removing agents.
  """
  def scale_swarm(target_count) do
    GenServer.call(__MODULE__, {:scale_swarm, target_count})
  end

  # GenServer callbacks

  @impl true
  def init(_opts) do
    # Start with some initial agents
    initial_agents = 3
    Enum.each(1..initial_agents, fn i ->
      OmokodaSwarm.SwarmSupervisor.start_agent("agent_#{i}")
    end)

    state = %__MODULE__{
      active_tasks: %{},
      agent_states: %{}
    }

    {:ok, state}
  end

  @impl true
  def handle_call({:submit_task, task, options}, _from, state) do
    task_id = generate_task_id()

    # Delegate task using delegation module
    case OmokodaSwarm.Delegation.delegate_task(task, options) do
      :ok ->
        new_active_tasks = Map.put(state.active_tasks, task_id, %{task: task, status: :delegated})
        {:reply, {:ok, task_id}, %{state | active_tasks: new_active_tasks}}

      error ->
        {:reply, error, state}
    end
  end

  @impl true
  def handle_call(:get_status, _from, state) do
    agents = OmokodaSwarm.SwarmSupervisor.list_agents()
    agent_statuses = Enum.map(agents, fn agent_id ->
      case OmokodaSwarm.Agent.get_state(agent_id) do
        {:ok, agent_state} -> {agent_id, agent_state}
        _ -> {agent_id, :unknown}
      end
    end)

    status = %{
      active_agents: length(agents),
      active_tasks: map_size(state.active_tasks),
      agent_statuses: agent_statuses
    }

    {:reply, status, state}
  end

  @impl true
  def handle_call({:scale_swarm, target_count}, _from, state) do
    current_count = length(OmokodaSwarm.SwarmSupervisor.list_agents())

    cond do
      target_count > current_count ->
        # Add agents
        Enum.each((current_count + 1)..target_count, fn i ->
          OmokodaSwarm.SwarmSupervisor.start_agent("agent_#{i}")
        end)

      target_count < current_count ->
        # Remove agents (simplified - just stop some)
        agents_to_remove = OmokodaSwarm.SwarmSupervisor.list_agents()
                          |> Enum.take(current_count - target_count)

        Enum.each(agents_to_remove, &OmokodaSwarm.SwarmSupervisor.stop_agent/1)

      true ->
        :ok
    end

    {:reply, :ok, state}
  end

  @impl true
  def handle_info({:task_completed, task_id, result}, state) do
    # Update task status
    updated_tasks = Map.update!(state.active_tasks, task_id, &Map.put(&1, :status, :completed))
    updated_tasks = Map.update!(updated_tasks, task_id, &Map.put(&1, :result, result))

    {:noreply, %{state | active_tasks: updated_tasks}}
  end

  defp generate_task_id do
    :crypto.strong_rand_bytes(8) |> Base.encode16()
  end
end