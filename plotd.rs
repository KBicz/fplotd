use std::process::exit;
use gnuplot::{Figure, Caption, Color, Graph, AxesCommon, PointSymbol, PointSize, LineWidth, Fix, Font, LineStyle, Solid, SmallDot, Dot, Dash, DotDash, DotDotDash, AutoOption::Auto, TickOption::{MajorScale, MinorScale}};

pub fn fplot(x: Vec<Vec<f64>>, y: Vec<Vec<f64>>, errx: Vec<Vec<f64>>, erry: Vec<Vec<f64>>, xmin: f64, xmax: f64, ymin: f64, ymax: f64, xw: u32, yw: u32, fonts: f64, fontx: f64, fonty: f64, fontt: f64, xwid: u32, ywid: u32, xpos: f64, ypos: f64, xlabel: String, ylabel: String, tit: String, lsty: String, syms: char, mars: f64, thk: f64, legx: f64, legy: f64, legctrl: bool, minctrl: bool, maxctrl: bool, xmino: f64, xmaxo: f64, ymino: f64, ymaxo: f64, pal: bool, linectrl: bool, errxctrl: bool, erryctrl: bool, legendstr: Vec<&str>, savectrl: bool, sformat: String, savename: &str, qt: bool, hlinex: Vec<Vec<f64>>, hliney: Vec<Vec<f64>>, vlinex: Vec<Vec<f64>>, vliney: Vec<Vec<f64>>) -> Result<(), &'static str> 
{
    let mut col: &str;
    let linesty;
    let mut fig = Figure::new();
    let colors: Vec<&str> = vec!["#1f77b4","#d62728","#2ca02c","#ff7f0e","#9467bd","#8c564b","#e377c2","#7f7f7f","#bcbd22","#17becf"];
    let mut tf = "x11";
    
    if qt {tf = "qt";}
    if lsty.eq("Solid") {linesty = Solid;}
    else if lsty.eq("SmallDot") {linesty = SmallDot;}
    else if lsty.eq("Dot") {linesty = Dot;}
    else if lsty.eq("Dash") {linesty = Dash;}
    else if lsty.eq("DotDash") {linesty = DotDash;}
    else if lsty.eq("DotDotDash") {linesty = DotDotDash;} 
    else {linesty = Solid;}

    if std::env::consts::OS.eq("linux") { let _a = fig.set_pre_commands(&format!("set term {} persist size {}, {} position {}, {}\nset tics font \", {}\"",tf,xwid,ywid,xpos,ypos,fonts)); }
    else { let _a = fig.set_pre_commands(&format!("set term qt persist size {}, {} position {}, {}\nset tics font \", {}\"",xwid,ywid,xpos,ypos,fonts)); }

    let ax = fig.axes2d();

    ax.set_title(&tit, &[Font("Arial", fontt)]);
    ax.set_x_label(&xlabel, &[Font("Arial", fontx)]);
    ax.set_y_label(&ylabel, &[Font("Arial", fonty)]);

    for i in 0..x.len()
    {
        col = colors[i%colors.len()];
        if erryctrl { ax.y_error_bars(x[i].clone(),y[i].clone(),erry[i].clone(),&[PointSymbol('O'),PointSize(0.0),Color("black")]); }
        if errxctrl { ax.x_error_bars(x[i].clone(),y[i].clone(),errx[i].clone(),&[PointSymbol('O'),PointSize(0.0),Color("black")]); }
        if linectrl 
        { 
            if legctrl { ax.lines(x[i].clone(),y[i].clone(),&[Caption(&legendstr[i].replace("_","\\_")),LineStyle(linesty),Color(col),LineWidth(thk)]); }
            else { ax.lines(x[i].clone(),y[i].clone(),&[LineStyle(linesty),Color(col),LineWidth(thk)]); }
        }
        else if pal 
        {
            ax.lines(x[i].clone(),y[i].clone(),&[LineStyle(linesty),Color(col),LineWidth(thk)]); 
            if legctrl { ax.points(x[i].clone(),y[i].clone(),&[Color(col),PointSymbol(syms),PointSize(mars),Caption(&legendstr[i].replace("_","\\_"))]); }
            else { ax.points(x[i].clone(),y[i].clone(),&[Color(col),PointSymbol(syms),PointSize(mars)]); }
        }
        else
        {
            if legctrl { ax.points(x[i].clone(),y[i].clone(),&[Color(col),PointSymbol(syms),PointSize(mars),Caption(&legendstr[i].replace("_","\\_"))]); }
            else { ax.points(x[i].clone(),y[i].clone(),&[Color(col),PointSymbol(syms),PointSize(mars)]); }
        }
    }
    
    for i in 0..hlinex.len()
    {
        ax.lines(hlinex[i].clone(),hliney[i].clone(),&[LineStyle(linesty),Color("black"),LineWidth(thk)]);
    }
    for i in 0..vlinex.len()
    {
        ax.lines(vlinex[i].clone(),vliney[i].clone(),&[LineStyle(linesty),Color("black"),LineWidth(thk)]);
    }

    if minctrl { ax.points(vec![xmino],vec![ymino],&[Color("#e60000"),PointSymbol(syms),PointSize(mars),Caption(&format!("Minimal value = {}",ymino))]); }
    if maxctrl { ax.points(vec![xmaxo],vec![ymaxo],&[Color("#a64dff"),PointSymbol(syms),PointSize(mars),Caption(&format!("Maximal value = {}",ymaxo))]); }

    if legctrl { ax.set_legend(Graph(legx), Graph(legy), &[], &[]); }
    ax.set_x_range(Fix(xmin),Fix(xmax));
    ax.set_y_range(Fix(ymin),Fix(ymax));
    ax.set_x_ticks(Some((Auto, 4)), &[MajorScale(1.5), MinorScale(0.75)],&[]);
    ax.set_y_ticks(Some((Auto, 4)), &[MajorScale(1.5), MinorScale(0.75)],&[]);

    if savectrl
    {
        if sformat.eq("pdf") { let _a = fig.save_to_pdf(savename, xw, yw);}
        else if sformat.eq("eps") { let _a = fig.save_to_eps(savename, xw, yw);}
        else if sformat.eq("png") { let _a = fig.save_to_png(savename, xwid, ywid);}
        else if sformat.eq("svg") { let _a = fig.save_to_pdf(savename, xwid, ywid);}
        else {exit(1);}
    }

    let _a = fig.show();
    Ok(())
}