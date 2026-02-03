use crate::spring::SpringConfig;

pub fn spring_fast() -> SpringConfig {
    SpringConfig {
        stiffness: 200.0,
        damping: 16.0,
        mass: 1.0,
        precision: 0.001,
    }
}

pub fn spring_soft() -> SpringConfig {
    SpringConfig {
        stiffness: 280.0,
        damping: 20.0,
        mass: 1.0,
        precision: 0.001,
    }
}

pub fn spring_slide() -> SpringConfig {
    SpringConfig {
        stiffness: 300.0,
        damping: 30.0,
        mass: 1.0,
        precision: 0.01,
    }
}

pub fn spring_flip_3d() -> SpringConfig {
    SpringConfig {
        stiffness: 150.0,
        damping: 25.0,
        mass: 1.0,
        precision: 0.001,
    }
}
