use super::*;

pub struct TeXTable {}
pub struct TeXMatrix {}

struct WolframMatrix<'a> {
    matrix: &'a DMatrix<f64>,
    rationalize: bool,
    breakable: bool,
}

impl<'a> Display for WolframMatrix<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(r#"
plotPDF[p_] := Block[
    {pdf},
    pdf = Table[{j, PDF[FirstPassageTimeDistribution[p, i], j]}, {i, selected + 1}, {j, steps}];
    ListLinePlot[
        pdf, 
        PlotLabel -> "\:9996\:6b21\:8fbe\:6210\:6982\:7387", AxesLabel -> {"\:5f3a\:5316\:6b21\:6570", "\:8fbe\:6210\:6982\:7387"}, PlotLegends -> legends, 
        PlotRange -> Full, Mesh -> Full, PlotTheme -> "FullAxesGrid", PlotStyle -> 24
    ]
];

plotCDF[p_] := Block[
    {cdf},
    cdf = Table[{j, CDF[FirstPassageTimeDistribution[p, i], j]}, {i, selected + 1}, {j, steps}];
    ListLinePlot[
        cdf,
        PlotLabel -> "\:7d2f\:8ba1\:8fbe\:6210\:6982\:7387", AxesLabel -> {"\:5f3a\:5316\:6b21\:6570", "\:8fbe\:6210\:6982\:7387"}, PlotLegends -> legends,
        PlotRange -> Full, Mesh -> Full, PlotTheme -> "Scientific", PlotStyle -> 24
    ]
]

plotMean[p_, max_] := Block[
    {mean, m1, m2, m3, m4, m5},
    mean = Table[Mean @ FirstPassageTimeDistribution[p, i], {i, max}];
    m1 = Table[Quantile[FirstPassageTimeDistribution[p, i], 0.05], {i, max}];
    m2 = Table[Quantile[FirstPassageTimeDistribution[p, i], 0.25], {i, max}];
    m3 = Table[Quantile[FirstPassageTimeDistribution[p, i], 0.50], {i, max}];
    m4 = Table[Quantile[FirstPassageTimeDistribution[p, i], 0.75], {i, max}];
    m5 = Table[Quantile[FirstPassageTimeDistribution[p, i], 0.95], {i, max}];
    ListLinePlot[
        {mean, m1, m2, m3, m4, m5},
        AxesLabel -> {"\:5f3a\:5316\:7b49\:7ea7", "\:5f3a\:5316\:6b21\:6570"},
        PlotLegends -> {"\:5e73\:5747\:8fbe\:6210\:6b21\:6570", "5%\:8fbe\:6210\:6b21\:6570", "25%\:8fbe\:6210\:6b21\:6570", "\:4e2d\:4f4d\:8fbe\:6210\:6b21\:6570", "75%\:8fbe\:6210\:6b21\:6570", "95%\:8fbe\:6210\:6b21\:6570"},
        PlotStyle -> 114, PlotRange -> Full, Mesh -> Full, PlotTheme -> {"Default", "Grid"}
    ]
]

"#)?;

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
selected = Range[1,5];
legends = "+"<>ToString[#]&/@selected;
plotPDF[𝒫]
plotCDF[𝒫]
plotMean[𝒫, {row}]
"#,
                row = rows - 6
            )
        }
    }
}

impl EnhanceMatrix {
    pub fn as_wolfram(&self, rationalize: bool) -> String {
        let matrix = WolframMatrix { matrix: &self.matrix, rationalize, breakable: self.breakable };
        matrix.to_string()
    }
}
