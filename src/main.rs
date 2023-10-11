#![allow(unused)]

fn main() {
    println!("Hello, world!");
}

struct Beam {
    spans: Vec<Span>,
    nodes: Vec<Node>,
    loads: Vec<AppliedLoad>,
}
struct Span {
    span_type: SpanType,
    length: f32,
    e_value: f32,
    i_value: f32,
    ei_value: f32,
    support_conditions: Vec<SupportCondition>,
    applied_loads: AppliedLoad,
}

enum SpanType {
    FixedEnd,
    ProppedCantilever,
}

enum SupportCondition {
    InternalHinge,
    Fixed,
    Pinned,
    Roller,
}

struct Node {
    is_support: bool,
    is_loaded: bool,
    support_condition: Option<SupportCondition>,
    applied_load: Option<AppliedLoad>,
}

struct AppliedLoad {
    load_type: AppliedLoadType,
    magnitude: f32,
    x_value: f32,   // distance from node i, counting from left
    a_b_value: f32, // a-b span version i.e position on span
}

enum AppliedLoadType {
    PointLoad,
    UniformLoad,
    LinearlyVarying,
}

trait FixedEndForces {
    fn fef_calc(&self) -> Vec<f32>; // calculate fixed-end forces
}

trait FixedStructureDispl {
    // fixed structure displacements
    fn disp_calc(&self) -> Vec<f32>; // incl. EI and the like
}

trait Factors {
    fn stiff_factor(&self); // stiffness factor
    fn dist_factor(&self); // distribution factor
}

impl FixedEndForces for Span {
    fn fef_calc(&self) -> Vec<f32> {
        match self.span_type {
            SpanType::FixedEnd => {
                // fixed structure forces
                let fef_i = self.applied_loads.magnitude * self.length / -8.0;
                let fef_j = self.applied_loads.magnitude * self.length / 8.0;
                let fef_m = self.applied_loads.magnitude * self.length / 8.0; // mid-span moment

                // mid-span deflection
                let defl_mid = (self.applied_loads.magnitude * self.length.powf(3.0))
                    / (192.0 * self.e_value * self.i_value);

                let fixed_end_forces = vec![fef_i, fef_j, fef_m, defl_mid];
                println!("Fixed-forces: {:?}", fixed_end_forces);
            }
            SpanType::ProppedCantilever => {
                let fef_i = -3.0 * self.applied_loads.magnitude * self.length / 16.0;
                let fef_j: f32 = 0.0; // type annotation needed since rustc defaults to f64
                let fef_m = 5.0 * self.applied_loads.magnitude * self.length / 32.0; // mid-span moment

                // mid-span deflection
                let defl_mid = (7.0 * self.applied_loads.magnitude * self.length.powf(3.0))
                    / (768.0 * self.e_value * self.i_value);
                // propped-end slop
                let slope_pe = (self.applied_loads.magnitude * self.length.powf(2.0))
                    / (-32.0 * self.e_value * self.i_value);

                let fixed_end_forces = vec![fef_i, fef_j, fef_m, defl_mid, slope_pe];
            }
        }

        vec![0.0, 0.0]
    }
}

impl FixedEndForces for Beam {
    fn fef_calc(&self) -> Vec<f32> {
        // should take the consituent span's FEFs and combine them
        vec![0.0, 0.0]
    }
}
