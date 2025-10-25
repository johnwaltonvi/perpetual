const DEFAULT_LIMIT_GB: f32 = 4.0;

pub struct MemoryBudget {
    pub bytes: f32,
    pub from_config: bool,
}

pub fn resolve_memory_budget(limit_gb: Option<f32>) -> MemoryBudget {
    match limit_gb {
        Some(limit) => MemoryBudget {
            bytes: limit * 1e9_f32,
            from_config: true,
        },
        None => MemoryBudget {
            bytes: DEFAULT_LIMIT_GB * 1e9_f32,
            from_config: false,
        },
    }
}
