package main

import (
	"testing"
	"time"
)

func TestHealthStatus(t *testing.T) {
	health := GetHealth()

	if !health.Healthy {
		t.Errorf("Expected healthy=true, got %v", health.Healthy)
	}

	if health.UptimeSeconds < 0 {
		t.Errorf("Expected positive uptime, got %d", health.UptimeSeconds)
	}
}

func TestSetHealth(t *testing.T) {
	SetHealth(false)
	health := GetHealth()
	if health.Healthy {
		t.Errorf("Expected healthy=false after SetHealth(false)")
	}

	SetHealth(true)
	health = GetHealth()
	if !health.Healthy {
		t.Errorf("Expected healthy=true after SetHealth(true)")
	}
}

func TestIsReady(t *testing.T) {
	SetHealth(true)
	if !IsReady() {
		t.Errorf("Expected ready=true when healthy=true")
	}

	SetHealth(false)
	if IsReady() {
		t.Errorf("Expected ready=false when healthy=false")
	}

	SetHealth(true)
}

func TestNodeStatus(t *testing.T) {
	status := GetNodeStatus()

	if status.NodeID == "" {
		t.Errorf("Expected node ID, got empty string")
	}

	if status.ActiveAgents < 0 {
		t.Errorf("Expected non-negative active agents, got %d", status.ActiveAgents)
	}
}

func TestUpdateNodeStatus(t *testing.T) {
	UpdateNodeStatus(5, 10, 45.0, 35.0)
	status := GetNodeStatus()

	if status.ActiveAgents != 5 {
		t.Errorf("Expected 5 active agents, got %d", status.ActiveAgents)
	}

	if status.TaskQueue != 10 {
		t.Errorf("Expected 10 in task queue, got %d", status.TaskQueue)
	}
}

func TestRegisterAgent(t *testing.T) {
	// Clear agents first
	agents = make(map[string]*AgentInfo)

	RegisterAgent("agent-1", 50.0)
	count := GetActiveAgents()

	if count != 1 {
		t.Errorf("Expected 1 agent, got %d", count)
	}

	agent := GetAgentInfo("agent-1")
	if agent == nil {
		t.Errorf("Expected agent info, got nil")
	}

	if agent.Status != "idle" {
		t.Errorf("Expected status idle, got %s", agent.Status)
	}
}

func TestUnregisterAgent(t *testing.T) {
	agents = make(map[string]*AgentInfo)

	RegisterAgent("agent-1", 50.0)
	UnregisterAgent("agent-1")

	count := GetActiveAgents()
	if count != 0 {
		t.Errorf("Expected 0 agents, got %d", count)
	}
}

func TestUpdateAgentStatus(t *testing.T) {
	agents = make(map[string]*AgentInfo)

	RegisterAgent("agent-1", 50.0)
	UpdateAgentStatus("agent-1", "busy")

	agent := GetAgentInfo("agent-1")
	if agent.Status != "busy" {
		t.Errorf("Expected status busy, got %s", agent.Status)
	}
}

func TestRecordAgentTask(t *testing.T) {
	agents = make(map[string]*AgentInfo)

	RegisterAgent("agent-1", 50.0)
	RecordAgentTask("agent-1", true)
	RecordAgentTask("agent-1", true)
	RecordAgentTask("agent-1", false)

	agent := GetAgentInfo("agent-1")
	if agent.TasksCompleted != 2 {
		t.Errorf("Expected 2 completed tasks, got %d", agent.TasksCompleted)
	}

	if agent.FailureCount != 1 {
		t.Errorf("Expected 1 failure, got %d", agent.FailureCount)
	}
}

func TestGetAllAgents(t *testing.T) {
	agents = make(map[string]*AgentInfo)

	RegisterAgent("agent-1", 50.0)
	RegisterAgent("agent-2", 60.0)
	RegisterAgent("agent-3", 70.0)

	allAgents := GetAllAgents()
	if len(allAgents) != 3 {
		t.Errorf("Expected 3 agents, got %d", len(allAgents))
	}
}

func TestMetricsIncrement(t *testing.T) {
	// These are just basic sanity checks
	IncrementTasksCompleted()
	IncrementReceiptsGenerated()
	IncrementReputationUpdate()

	// If we get here without panic, the tests pass
	t.Log("Metrics incremented successfully")
}

func TestMetricSnapshot(t *testing.T) {
	UpdateNodeStatus(3, 5, 50.0, 40.0)
	snapshot := GetMetricSnapshot()

	if snapshot.ActiveAgents != 3 {
		t.Errorf("Expected 3 active agents, got %d", snapshot.ActiveAgents)
	}

	if snapshot.TaskQueueLength != 5 {
		t.Errorf("Expected task queue 5, got %d", snapshot.TaskQueueLength)
	}
}

func TestAgentStaleDetection(t *testing.T) {
	agents = make(map[string]*AgentInfo)

	// Register an agent
	RegisterAgent("agent-1", 50.0)

	// Manually update the agent's last active time to be stale
	agentsMutex.Lock()
	agent := agents["agent-1"]
	agent.LastActiveTime = time.Now().Add(-6 * time.Minute)
	agentsMutex.Unlock()

	// Run the stale check
	checkStaleAgents()

	// Check if agent is marked as stale
	updatedAgent := GetAgentInfo("agent-1")
	if updatedAgent.Status != "stale" {
		t.Errorf("Expected stale status, got %s", updatedAgent.Status)
	}
}
