pub fn ln(tab: &Vec<f64>) -> Vec<f64>
{
    let mut val: f64;
    let mut tab2 = vec![0.0; tab.len()];
    for i in 0..tab.len()
    {
        if tab[i] == 0.0 { val = 1f64.powf(-6.0); }
        else { val = tab[i]; }
        tab2[i] = val.ln();
    }
    return tab2;
}