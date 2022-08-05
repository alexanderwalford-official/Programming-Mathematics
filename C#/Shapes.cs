/*
    I'm using this file to cover the basic fundlementals of shapes in mathematics
    and to convert it into code so that I can implement features without thought.
*/


public static class Shapes () {

    // 2D Shapes - Practical Examples

    void main () {

        // example 1:
        // Task: find the area of a circle

        // solution: pi times by radius squared

        float pi = 3.141592;
        float radius = 1.2345; // radius of circle
        float area = pi * radius * radius; // multiply pi by radius squared


        // example 2:
        // Task: find the hypotenuse of a right-angled triangle (the missing value of a side)

        // solution: a squared plus b squared equals c squared

        float a = 1.2345; // side a
        float b = 2.3456; // side b
        float c; // side c (no value)
        c = a * a + b * b; // a squared plus b squared
        c = c * c; // c squared (final answer)


        // example 3:
        // Task: Find the area of triangle

        // solution: height multiplied by width (base), then devide by 2

        float height = 1.2345;
        float width = 1.2345;
        float area = height * width / 2;


        // example 4:
        // Task: Find the missing angle in a triangle

        // solution: angle a plus angle b, subtract from 180

        float angleA = 57.65;
        float angleB = 23.35;
        float angleC = angleA + angleB - 180;
    }
}

