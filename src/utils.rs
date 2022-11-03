pub fn deg_to_rad(degrees: f32) -> f32 {
    return degrees * (std::f32::consts::PI / 180.0);
}

pub fn calc_world_curve_path(hor_position: f32) -> f32 {
    // Before you jenk with these calculations, use Desmos.com to visualize the
    // curvature of what you're trying to achieve.
    // As of right now "visible" chunks are min of y = -1.3 and size 50 ground means
    // -25 < x < 25
    // 25 < x < 75

    // let a = -1.0 / 100.0; // "Squash Facor" -- <1 is wider >1 is narrower
    // let b = -1.0 / 2.0; // Horizontal shift is -0.5 the b value so we double it here for you.
    // let c = 18.75; // Vertical shift value

    let a = -1.0 / 300.0; // "Squash Facor" -- <1 is wider >1 is narrower
    let b = 1.0 / 8.0; // Horizontal shift is -0.5 the b value so we double it here for you.
    let c = 12.0; // Vertical shift value
    
    let ground_height = -1.3;
    let parab_path = (a * (hor_position * hor_position)) + (b * hor_position) + (c);

    return min(parab_path, ground_height);
}

// This guys is needed because floats don't support Ord so we can't use the builtins
pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        return a;
    } else {
        return b ;
    }
}