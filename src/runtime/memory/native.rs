use sysinfo::System;

pub struct MemoryBudget {
    pub bytes: f32,
    pub from_config: bool,
}

pub fn resolve_memory_budget(limit_gb: Option<f32>) -> MemoryBudget {
    if let Some(limit) = limit_gb {
        return MemoryBudget {
            bytes: limit * 1e9_f32,
            from_config: true,
        };
    }

    let system = System::new_all();
    let estimated = system
        .cgroup_limits()
        .map(|limits| limits.free_memory as f32)
        .unwrap_or_else(|| system.available_memory() as f32);

    MemoryBudget {
        bytes: estimated,
        from_config: false,
    }
}
