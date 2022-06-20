//CREATE ClASS
    class Calculator {   
        constructor() {
            this.displayValue = '';
        }//end constructor();


    }//end function

//CREATE INSTANCE OF THE CLASS THIS INSTANCE IS GLOBAL
    webCalculator = new Calculator;

//EVENTS
    function InputDigit(digit) {
            webCalculator.displayValue += digit;
            RefreshScreen();
    }//end function

    function InputOperator(digit) {
        webCalculator.displayValue += " " + digit + " ";
        RefreshScreen();
    }//end function

    function ResetCalculator() {
        webCalculator.displayValue = '';
        RefreshScreen();
    }//end function

//ALTER THE CALCULATOR'S SCREEN
    function RefreshScreen() {
        //GRAB THE ELEMENT THAT REPRESENTS THE SCREEN <INPUT>
        var inputElement = document.querySelector('.calculator-screen');

        //CHANGE THE VALUE PROPERTY OF THE ELEMENT
        inputElement.value = webCalculator.displayValue;
    }//end function

//PROCESS INPUT ON CALCULATOR SCREEN
    function ProcessInput() {
        // webCalculator.displayValue = eval(webCalculator.displayValue);

        var answer = InfixToPostfix(webCalculator.displayValue);

        webCalculator.displayValue = PostFixEvaluate(answer);

        RefreshScreen();
    }//end function


    // Used in conversion
    function Precedence(c)
    {
        switch (c)
        {
            case '^':
                return 3;

            case '/':
            case '*':
                return 2;

            case '+':
            case '-':
                return 1;

            default:
                return -1;
        }
    }


    function InfixToPostfix(input)
    {
        output = "";
        stack = [];

        // scan expression from left to right
        for (i = 0; i < input.length; i++)
        {
            c = input[i];

            // handles the spacing
            if (c == ' ')
            {
                // if the output already has a space, skip
                if (output[output.length - 1] == ' ')
                {
                    continue;
                }
                // else add a space to the output

                output += ' ';
                continue;
            }

            // if char is a operand add to output
            if (c >= '0' && c <= '9')
            {
                output += c;

            }

            // else if is a (, push
            else if (c == '(')
            {
                stack.push(c);
            }

            // if closing, pop and add to output until )
            else if (c == ')')
            {
                while (stack.peek() != '(' && stack.length > 0)
                {// this doesnt seem right...
                    output += ' ';
                    output += stack.pop();
                    output += ' ';
                }

                // if the stack is empty and a ( was never reached, throw error
                if (stack.peek() != '(' && stack.length > 0)
                {
                    return "Invalid Expression";
                }

                // pop the (
                else
                {
                    stack.pop();
                }
            }
            // if its a operator
            else
            {
                // while the precendence is less than what is than stack
                while (stack.length > 0 && Precedence(c) <= Precedence(stack[stack.length-1]))
                {
                    // add the higher precedence operators to the output
                    output += stack.pop();
                }
                // push operator the stack
                stack.push(c);
            }
        }//endfor
        // gone thru entire expression

        // Pop remaining and add to output
        while (stack.length > 0)
        {
            output += ' ';
            output += stack.pop();
            output += ' ';
        }

        return output;
    }


    function PostFixEvaluate(input)
    {
        stack = [];
        if (input == "")
        {
            return 0;
        }

        // split method splits a string up by the given char
        tokens = input.split(' ');

        operation = "";

        // for each character in the input
        for (i = 0; i < tokens.length; i++)
        {
            if (tokens[i] == "" || tokens[i] == " ")
            {
                continue;
            }
            // if input is a number add to stack
            if (num = parseInt(tokens[i], 10))
            {
                stack.push(num);
                operation = "push";
            }
            // else if its a operator, perform operation
            else
            {
                n1 = stack.pop();
                n2 = stack.pop();

                if (tokens[i] == "+")
                {
                    stack.push(n1 + n2);
                    operation = "+";
                }
                else if (tokens[i] == "-")
                {
                    stack.push(n2 - n1);
                    operation = "-";
                }
                else if (tokens[i] == "*")
                {
                    stack.push(n2 * n1);
                    operation = "*";
                }
                else if (tokens[i] == "/")
                {
                    stack.push(n2 / n1);
                    operation = "/";
                }
            }
        }
        return stack[stack.length-1];
    }
