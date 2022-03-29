Rust Interpreter for bytecode 

The bytecode language it's supporting the basic arithmatic and loop.

For reference used the following codes :- 

 1) Basic arithmatic:-
 
 function f() {

        x = 1                   LOAD_VAL 1
                                WRITE_VAR ‘x’

        y = 2                   LOAD_VAL 2
                                WRITE_VAR ‘y’

        return (x + 1) * y      READ_VAR ‘x’
                                LOAD_VAL 1
                                ADD

                                READ_VAR ‘y’
                                MULTIPLY

                                RETURN_VALUE
    }
    
    
  2) Loop 
    
    function f() {
        sum = 0				      	
        for ( x = 0; x < 10; x++ )	{					
			  sum = sum + x				
		  }					
		return y				
    }
    
    // sum = 0
	LOAD_VAL 0
	WRITE_VAR 'sum'

	// x = 0
	LOAD_VAL 0
	WRITE_VAR 'x'
	JMP LOOP_TEST

	// {body} + {inc/dec}
	LABEL LOOP_BODY
	// sum = sum + x
	READ_VAR 'sum'
	READ_VAR 'x'
	ADD
	WRITE_VAR 'sum'

	// x++
	LOAD_VAL 1
	READ_VAR 'x'
	ADD
	WRITE_VAR 'x'

	// x < 10
	LABEL LOOP_TEST
	READ_VAR 'x'
	CMP 10
	JMP_LE LOOP_BODY

	// return sum
	READ_VAR 'sum'
	RETURN_VALUE


    
    
