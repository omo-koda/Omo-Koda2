package main

import (
	"sync"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
)

var (
	// Agent metrics
	activeAgentsGauge = promauto.NewGauge(prometheus.GaugeOpts{
		Name: "omokoda_active_agents",
		Help: "Number of active agents in the swarm",
	})

	agentCreatedCounter = promauto.NewCounter(prometheus.CounterOpts{
		Name: "omokoda_agents_created_total",
		Help: "Total number of agents created",
	})

	agentDestroyedCounter = promauto.NewCounter(prometheus.CounterOpts{
		Name: "omokoda_agents_destroyed_total",
		Help: "Total number of agents destroyed",
	})

	// Task metrics
	taskQueueGauge = promauto.NewGauge(prometheus.GaugeOpts{
		Name: "omokoda_task_queue_length",
		Help: "Current length of the task queue",
	})

	tasksCompletedCounter = promauto.NewCounter(prometheus.CounterOpts{
		Name: "omokoda_tasks_completed_total",
		Help: "Total number of tasks completed",
	})

	taskDurationHistogram = promauto.NewHistogram(prometheus.HistogramOpts{
		Name:    "omokoda_task_duration_seconds",
		Help:    "Task execution duration in seconds",
		Buckets: prometheus.DefBuckets,
	})

	// Resource metrics
	cpuUsageGauge = promauto.NewGauge(prometheus.GaugeOpts{
		Name: "omokoda_cpu_usage_percent",
		Help: "CPU usage percentage",
	})

	memoryUsageGauge = promauto.NewGauge(prometheus.GaugeOpts{
		Name: "omokoda_memory_usage_percent",
		Help: "Memory usage percentage",
	})

	// Receipt metrics
	receiptsGeneratedCounter = promauto.NewCounter(prometheus.CounterOpts{
		Name: "omokoda_receipts_generated_total",
		Help: "Total number of receipts generated",
	})

	receiptVerificationFailureCounter = promauto.NewCounter(prometheus.CounterOpts{
		Name: "omokoda_receipt_verification_failures_total",
		Help: "Total number of receipt verification failures",
	})

	// Reputation metrics
	reputationUpdateCounter = promauto.NewCounter(prometheus.CounterOpts{
		Name: "omokoda_reputation_updates_total",
		Help: "Total number of reputation updates",
	})

	// Lock for metrics updates
	metricsMutex sync.RWMutex
)

// InitializeMetrics initializes all metrics
func InitializeMetrics() {
	// Start metric collection loop
	go metricCollectionLoop()
}

// MetricSnapshot holds a snapshot of current metrics
type MetricSnapshot struct {
	ActiveAgents               int64
	TaskQueueLength            int64
	TasksCompleted             int64
	CPUUsagePercent            float64
	MemoryUsagePercent         float64
	ReceiptsGenerated          int64
	ReceiptVerificationFailure int64
	ReputationUpdates          int64
}

func GetMetricSnapshot() MetricSnapshot {
	metricsMutex.RLock()
	defer metricsMutex.RUnlock()

	return MetricSnapshot{
		ActiveAgents:       GetActiveAgents(),
		TaskQueueLength:    GetTaskQueueLength(),
		CPUUsagePercent:    getCPUUsagePercent(),
		MemoryUsagePercent: getMemoryPercent(),
	}
}

// Agent metrics functions
func IncrementAgentCreated() {
	agentCreatedCounter.Inc()
	activeAgentsGauge.Inc()
}

func IncrementAgentDestroyed() {
	agentDestroyedCounter.Inc()
	activeAgentsGauge.Dec()
}

func SetActiveAgents(count int64) {
	activeAgentsGauge.Set(float64(count))
}

// Task metrics functions
func SetTaskQueueLength(length int64) {
	taskQueueGauge.Set(float64(length))
}

func IncrementTasksCompleted() {
	tasksCompletedCounter.Inc()
}

func ObserveTaskDuration(duration float64) {
	taskDurationHistogram.Observe(duration)
}

// Receipt metrics functions
func IncrementReceiptsGenerated() {
	receiptsGeneratedCounter.Inc()
}

func IncrementReceiptVerificationFailure() {
	receiptVerificationFailureCounter.Inc()
}

// Reputation metrics functions
func IncrementReputationUpdate() {
	reputationUpdateCounter.Inc()
}

func metricCollectionLoop() {
	// Placeholder for periodic metric collection
	// In a real implementation, this would collect system metrics
	// and update gauges regularly
}

func getCPUUsagePercent() float64 {
	// In a real implementation, this would read from /proc/stat
	// For now, return a simulated value
	return 35.5
}
