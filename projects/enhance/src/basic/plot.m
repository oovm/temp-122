plotPDF[p_] := Block[
    {legends, pdf},
    legends = "+"<>ToString[#]&/@selected;
    pdf = Table[{j, PDF[FirstPassageTimeDistribution[p, i], j]}, {i, selected + 1}, {j, steps}];
    ListLinePlot[
        pdf,
        PlotLabel -> "\:9996\:6b21\:8fbe\:6210\:6982\:7387", AxesLabel -> {"\:5f3a\:5316\:6b21\:6570", "\:8fbe\:6210\:6982\:7387"}, PlotLegends -> legends,
        PlotRange -> Full, Mesh -> Full, PlotTheme -> "FullAxesGrid", PlotStyle -> 24
    ]
];

plotCDF[p_] := Block[
    {legends, cdf},
    legends = "+"<>ToString[#]&/@selected;
    cdf = Table[{j, CDF[FirstPassageTimeDistribution[p, i], j]}, {i, selected + 1}, {j, steps}];
    ListLinePlot[
        cdf,
        PlotLabel -> "\:7d2f\:8ba1\:8fbe\:6210\:6982\:7387", AxesLabel -> {"\:5f3a\:5316\:6b21\:6570", "\:8fbe\:6210\:6982\:7387"}, PlotLegends -> legends,
        PlotRange -> Full, Mesh -> Full, PlotTheme -> "Scientific", PlotStyle -> 24
    ]
]

plotMean[p_, max_] := Block[
    {mean, m1, m2, m3, m4, m5},
    mean = Table[{i, Mean@FirstPassageTimeDistribution[p, i + 1]}, {i, 0, max}];
    m1 = Table[{i, Quantile[FirstPassageTimeDistribution[p, i + 1], 0.05]}, {i, 0, max}];
    m2 = Table[{i, Quantile[FirstPassageTimeDistribution[p, i + 1], 0.25]}, {i, 0, max}];
    m3 = Table[{i, Quantile[FirstPassageTimeDistribution[p, i + 1], 0.50]}, {i, 0, max}];
    m4 = Table[{i, Quantile[FirstPassageTimeDistribution[p, i + 1], 0.75]}, {i, 0, max}];
    m5 = Table[{i, Quantile[FirstPassageTimeDistribution[p, i + 1], 0.95]}, {i, 0, max}];
    ListLinePlot[
        {mean, m1, m2, m3, m4, m5},
        AxesLabel -> {"\:5f3a\:5316\:7b49\:7ea7", "\:5f3a\:5316\:6b21\:6570"},
        PlotLegends -> {"\:5e73\:5747\:8fbe\:6210\:6b21\:6570", "5%\:8fbe\:6210\:6b21\:6570", "25%\:8fbe\:6210\:6b21\:6570", "\:4e2d\:4f4d\:8fbe\:6210\:6b21\:6570", "75%\:8fbe\:6210\:6b21\:6570", "95%\:8fbe\:6210\:6b21\:6570"},
        PlotStyle -> 114, PlotRange -> Full, Mesh -> Full, PlotTheme -> {"Default", "Grid"}
    ]
]
