
rtFn = "";
vtStk = "";
rtFnRefStk = "0";

rtFc = 0;
rtStkRef = 0;

a_gs1Rules = "\x01\x0201\x0B\x10"

v_currentCharIndex = 0;
v_output = "";
v_twoChars = "";
v_threeChars = "";

while (rtFn != "_entry") {
    if (rtFn == "")
    {
        rtFn = "_entry"
        rtFn = "_entry_0"
        rtFnStkRef = rtFnStkRef + "_"
        rtFc = rtFc + 1;
    }
    else if (rtFn == "_entry_0")
    {
        if (v_currentCharIndex < StrLen(OUT1.Data))
        {
            rtFn = "_entry_0"
        }
    }


    else if (rtFn == "parseNextGs1Code")
    {
        v_ruleFound = 0;
        v_ruleIndex = 0;

        v_foreach_rule_in_gs1Rules = 0;
        while (v_foreach_rule_in_gs1Rules < 3)
        {
            v_foreach_rule_value = "";
            v_foreach_rule_iterator = 0;
            while (v_foreach_rule_iterator < )

            v_foreach_rule_in_gs1Rules = v_foreach_rule_in_gs1Rules + 1;
        }
    }
}