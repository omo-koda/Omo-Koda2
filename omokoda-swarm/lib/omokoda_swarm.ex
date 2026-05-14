defmodule OmokodaSwarm do
  @moduledoc """
  Omokoda Swarm - Distributed Agent Orchestration System

  This module provides the main API for interacting with the swarm of agents.
  """

  @doc """
  Starts the swarm application.
  """
  def start do
    Application.start(:omokoda_swarm)
  end

  @doc """
  Submits a task to the swarm for processing.

  ## Examples

      iex> {:ok, task_id} = OmokodaSwarm.submit_task(%{type: :think, prompt: "Hello world"})
      iex> is_binary(task_id)
      true

  """
  def submit_task(task, options \\ []) do
    OmokodaSwarm.Coordinator.submit_task(task, options)
  end

  @doc """
  Gets the current status of the swarm.
  """
  def status do
    OmokodaSwarm.Coordinator.get_status()
  end

  @doc """
  Scales the swarm to the specified number of agents.
  """
  def scale_to(count) do
    OmokodaSwarm.Coordinator.scale_swarm(count)
  end

  @doc """
  Delegates a task to a specific agent.
  """
  def delegate_to_agent(agent_id, task) do
    OmokodaSwarm.Agent.delegate_task(agent_id, task)
  end

  @doc """
  Performs witness consensus on a task.
  """
  def witness_consensus(task, witness_count \\ 3) do
    OmokodaSwarm.Delegation.delegate_for_consensus(task, witness_count)
  end

  @doc """
  Lists all active agents in the swarm.
  """
  def list_agents do
    OmokodaSwarm.SwarmSupervisor.list_agents()
  end

  @doc """
  Gets the state of a specific agent.
  """
  def agent_state(agent_id) do
    OmokodaSwarm.Agent.get_state(agent_id)
  end
end
