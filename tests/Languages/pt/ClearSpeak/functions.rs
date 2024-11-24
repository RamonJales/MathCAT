/// Tests for:
/// *  functions including trig functions, logs, and functions to powers
/// *  implied times/functional call and explicit times/function call
/// *  parens
/// These are all intertwined, so they are in one file
use crate::common::*;

#[test]
fn trig_names() {
    let expr = "<math><mrow>
    <mi>sin</mi><mi>x</mi><mo>+</mo>
    <mi>cos</mi><mi>y</mi><mo>+</mo>
    <mi>tan</mi><mi>z</mi><mo>+</mo>
    <mi>sec</mi><mi>&#x03B1;</mi><mo>+</mo>
    <mi>csc</mi><mi>&#x03D5;</mi><mo>+</mo>
    <mi>cot</mi><mi>&#x03C6;</mi>
    </mrow></math>";
    test("pt", "ClearSpeak", expr, "seno de x mais cosseno de y mais tangente de z mais secante de alfa, mais cosecante de phi, mais cotangente de phi");
}

#[test]
fn hyperbolic_trig_names() {
    let expr = "<math><mrow>
    <mi>sinh</mi><mi>x</mi><mo>+</mo>
    <mi>cosh</mi><mi>y</mi><mo>+</mo>
    <mi>tanh</mi><mi>z</mi><mo>+</mo>
    <mi>sech</mi><mi>&#x03B1;</mi><mo>+</mo>
    <mi>csch</mi><mi>&#x03D5;</mi><mo>+</mo>
    <mi>coth</mi><mi>&#x03C6;</mi>
    </mrow></math>";
    test("pt", "ClearSpeak", expr, "seno hiperbólico de x, mais \
                                cosseno hiperbólico de y, mais \
                                tangente hiperbólica de z, mais \
                                secante hiperbólica de alfa, mais, \
                                cosecante hiperbólica de phi, mais, \
                                cotangente hiperbólica de phi");
}


//wrong wansser: "inverso seno de x"
// perhaps is should be better "seno na menos 1 de x?"
#[test]
fn inverse_trig() {
    let expr = "<math><msup><mi>sin</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("pt", "ClearSpeak", expr, "seno inverso de x");
}

//wrong wansser: returning "inverso", but is "inversa"
#[test]
fn inverse_trig_trig_inverse() {
    let expr = "<math><msup><mi>tan</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test_ClearSpeak("pt", "ClearSpeak_Trig", "TrigInverse",expr,
        "tangente inversa de x");
}

//wrong answer: "a quarto potência, secante de x"
#[test]
fn trig_fourth() {
    let expr = "<math><msup><mi>sec</mi><mn>4</mn></msup><mi>x</mi></math>";
    test("pt", "ClearSpeak", expr, "a quarta potência de secante de x");
}

#[test]
fn simple_log() {
    let expr = "<math> <mrow>  <mi>log</mi><mi>x</mi></mrow> </math>";
    test("pt", "ClearSpeak", expr, "logaritmo x");
}

#[test]
fn normal_log() {
    let expr = "<math><mrow><mi>log</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("pt", "ClearSpeak", expr, "o logaritmo de, abre parênteses, x mais y, fecha parênteses");
}

//wrong answer: "a base logaritímica b de x"
#[test]
fn simple_log_with_base() {
    let expr = "<math> <mrow>  <msub><mi>log</mi><mi>b</mi></msub><mi>x</mi></mrow> </math>";
    test("pt", "ClearSpeak", expr, "o logaritmo de x na base b");
}

#[test]
fn times_following_paren() {
    let expr = "<math><mrow>
        <mn>2</mn>
        <mrow>  <mo>(</mo> <mn>3</mn>  <mo>)</mo> </mrow>
        </mrow></math>";
    test("pt", "ClearSpeak", expr, "2 vezes 3");
}

#[test]
fn times_preceding_paren() {
    let expr = "<math><mrow>
        <mrow>  <mo>(</mo> <mn>2</mn>  <mo>)</mo> </mrow>
        <mn>3</mn>
        </mrow></math>";
    test("pt", "ClearSpeak", expr, "2 vezes 3");
}

#[test]
fn more_implied_times() {
    let expr = "<math><mrow>
    <mrow>
    <msup>
        <mrow>
        <mrow><mo>(</mo>
        <mrow> <mn>2</mn><mi>x</mi></mrow>
        <mo>)</mo></mrow></mrow>
        <mn>2</mn>
    </msup>
    </mrow>
    </mrow></math>";

    test_ClearSpeak("pt", "ClearSpeak_ImpliedTimes", "MoreImpliedTimes",expr,
        "abre parênteses, 2 vezes x, fecha parênteses ao quadrado");
}

#[test]
fn explicit_times_more_implied_times() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test_ClearSpeak("pt", "ClearSpeak_ImpliedTimes", "MoreImpliedTimes",expr, "t vezes x");
}

#[test]
fn explicit_times_none_simple_right() {
    let expr = "<math><mn>2</mn><mo>[</mo><mn>3</mn> <mo>]</mo></math>";
    test_ClearSpeak("pt", "ClearSpeak_ImpliedTimes", "None",
        expr, "2, abre colchetes 3 fecha colchetes");
}

#[test]
fn explicit_times_none_simple_left() {
    let expr = "<math><mo>(</mo><mn>2</mn><mo>&#x2212;</mo><mn>1</mn><mo>)</mo><mi>x</mi></math>";
    test_ClearSpeak("pt", "ClearSpeak_ImpliedTimes", "None",
        expr, "abre parênteses, 2 menos 1, fecha parênteses; x");
}

#[test]
fn explicit_times_none_superscript() {
    let expr = "<math> 
    <mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><msup>
<mi>x</mi>
<mn>2</mn>
</msup>
<mrow><mo>(</mo>
<mrow>
<mi>x</mi><mo>+</mo><mn>1</mn></mrow>
<mo>)</mo></mrow>
    </math>";
    test_ClearSpeak_prefs("pt", 
        vec![("ClearSpeak_ImpliedTimes", "None"), ("ClearSpeak_Functions", "None")],
        expr, "f, abre parênteses x fecha parênteses; é igual a; x ao quadrado, abre parênteses, x mais 1, fecha parênteses");
}

//Tests for parens
#[test]
fn no_parens_number() {
    let expr = "<math><mrow>
    <mrow><mo>(</mo>
    <mn>25</mn>
    <mo>)</mo></mrow>
    <mi>x</mi>
    </mrow></math>";
    test("pt", "ClearSpeak", expr, "25 vezes x");
}

#[test]
fn no_parens_monomial() {
    let expr = "<math><mrow>
    <mi>b</mi>
    <mrow><mo>(</mo>
    <mrow><mi>x</mi><mi>y</mi></mrow>
    <mo>)</mo></mrow>
    </mrow></math>";
    test("pt", "ClearSpeak", expr, "b x y");
}

#[test]
fn no_parens_negative_number() {
    let expr = "<math><mrow>
    <mn>2</mn><mo>+</mo>
    <mrow><mo>(</mo>
    <mrow><mo>&#x2212;</mo><mn>2</mn></mrow>
    <mo>)</mo></mrow>
    </mrow></math>";
    test("pt", "ClearSpeak", expr, "2 mais negativo 2");
}

#[test]
fn no_parens_negative_number_with_var() {
    let expr = "<math><mrow>
    <mrow><mo>(</mo>
    <mrow><mo>&#x2212;</mo><mn>2</mn></mrow><mi>x</mi>
    <mo>)</mo>
    </mrow>
    <mo>+</mo><mn>1</mn>
    </mrow></math>";
    test("pt", "ClearSpeak", expr, "negativo 2 x, mais 1");
}

#[test]
fn parens_superscript() {
    let expr = "<math><mrow>
    <mrow>
    <msup>
    <mrow>
        <mrow><mo>(</mo>
        <mrow> <mn>2</mn><mi>x</mi></mrow>
        <mo>)</mo></mrow></mrow>
    <mn>2</mn>
    </msup>
    </mrow>
    </mrow></math>";
    test("pt", "ClearSpeak", expr, "abre parênteses 2 x fecha parênteses ao quadrado");
}

#[test]
fn no_parens_fraction() {
    let expr = "<math><mrow>
    <mn>2</mn>
    <mo>+</mo>
    <mrow>
        <mrow><mo>(</mo>
        <mfrac> <mn>1</mn><mn>2</mn></mfrac>
        <mo>)</mo></mrow></mrow>
    </mrow></math>";
    test("pt", "ClearSpeak", expr, "2 mais 1 meio");
}

    // Tests for the ten types of intervals in ClearSpeak
    #[test]
    fn parens_interval_open_open() {
        let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("pt", "ClearSpeak_Paren", "Interval",expr,
    "o intervalo de c a d, não incluindo c ou d");
}

#[test]
    fn parens_interval_closed_open() {
        let expr = "<math> 
        <mrow><mo>[</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("pt", "ClearSpeak_Paren", "Interval ",expr,
    "o intervalo de c a d, incluindo c mas não incluindo d");
}

#[test]
fn parens_interval_open_closed() {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
    <mo>]</mo></mrow>
    </math>";
    test_ClearSpeak("en", "ClearSpeak_Paren", "Interval ",expr,
    "the interval from c to d, not including c but including d");
}

#[test]
fn parens_interval_closed_closed() {
    let expr = "<math> 
    <mrow><mo>[</mo>
    <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
    <mo>]</mo></mrow>
</math>";
test_ClearSpeak("pt", "ClearSpeak_Paren", "Interval ",expr,
"o intervalo de c a d, incluindo c e d");
}

#[test]
fn parens_interval_neg_infinity_open_open() {
    let expr = "<math> 
    <mrow><mo>(</mo>
    <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mi>d</mi></mrow>
    <mo>)</mo></mrow>
    </math>";
test_ClearSpeak("pt", "ClearSpeak_Paren", "Interval ",expr,
"o intervalo de negativo infinito a d, não incluindo d");
}

#[test]
fn parens_interval_neg_infinity_closed_open() {
    let expr = "<math> 
    <mrow><mo>(</mo>
    <mrow> <mo>-</mo> <mi>∞</mi><mo>,</mo><mi>d</mi></mrow>
    <mo>]</mo></mrow>
</math>";
test_ClearSpeak("pt", "ClearSpeak_Paren", "Interval ",expr,
"o intervalo de negativo infinito a d, incluindo d");
}

#[test]
fn parens_interval_open_open_infinity() {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>∞</mi></mrow>
    <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("pt", "ClearSpeak_Paren", "Interval ",expr,
    "o intervalo de c a infinito, não incluindo c");
}

#[test]
fn parens_interval_closed_open_infinity() {
    let expr = "<math> 
        <mrow><mo>[</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>∞</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("pt", "ClearSpeak_Paren", "Interval ",expr,
"o intervalo de c a infinito, incluindo c");
}

#[test]
fn parens_interval_neg_infinity_to_infinity() {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mi>∞</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("pt", "ClearSpeak_Paren", "Interval ",expr,
    "o intervalo de negativo infinito a infinito,");
}

#[test]
fn parens_interval_neg_infinity_to_pos_infinity() {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mo>+</mo><mi>∞</mi></mrow>
    <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("pt", "ClearSpeak_Paren", "Interval ",expr,
    "o intervalo de negativo infinito a positivo infinito,");
}
