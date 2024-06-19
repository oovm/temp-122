use super::*;

pub struct TeXTable {}
pub struct TeXMatrix {}

struct WolframMatrix<'a> {
    matrix: &'a DMatrix<f64>,
    rationalize: bool,
    breakable: bool,
    shared: bool,
}

impl<'a> Display for WolframMatrix<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.shared {
            f.write_str(include_str!("plot.m"))?;
        }
        f.write_str("ℳ = ")?;
        if self.rationalize {
            f.write_str("Rationalize@")?;
        }
        for (i, row) in self.matrix.row_iter().enumerate() {
            if i > 0 {
                f.write_str("},\n    {")?;
            }
            else {
                f.write_str("{\n    {")?;
            }

            for (j, value) in row.iter().enumerate() {
                if j > 0 {
                    f.write_str(", ")?;
                }
                write!(f, "{}", value)?;
            }
        }
        f.write_str("}\n};")?;
        if self.breakable {
            f.write_str("𝒫 = DiscreteMarkovProcess[2, ℳ];\n")?;
            writeln!(f, "Table[PDF[FirstPassageTimeDistribution[𝒫, 6], x], {{x, 0, 6*2}}]")
        }
        else {
            let rows = self.matrix.nrows();
            writeln!(
                f,
                r#"
𝒫 = DiscreteMarkovProcess[1, ℳ];

steps = Range[1,18];
selected = Range[1, {row}];

plotPDF[𝒫]
plotCDF[𝒫]
plotMean[𝒫, {row}]
"#,
                row = rows.saturating_sub(1)
            )
        }
    }
}

impl EnhanceMatrix {
    pub fn as_wolfram(&self, rationalize: bool, definition: bool) -> String {
        let matrix = WolframMatrix { matrix: &self.matrix, rationalize, breakable: self.breakable, shared: definition };
        matrix.to_string()
    }
}
