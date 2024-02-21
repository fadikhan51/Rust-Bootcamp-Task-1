<!DOCTYPE html>
<body>

<h1>Hands-on Task - Implement a basic program that uses ownership concepts</h1>

<h2>Task Details</h2>
<p>In this task, students will create a simple Rust program that demonstrates the concepts of ownership, borrowing, and references. The program will take two strings as input, concatenate them, and then print the result without violating any ownership rules.</p>

<h2>Steps</h2>

<ol>
  <li>Create a function called <code>concatenate_strings</code> that takes two string slices as arguments and returns a new String as the result of concatenating the two input strings.</li>
  <li>Inside the <code>concatenate_strings</code> function, create a new String called <code>result</code>. Use the <code>push_str()</code> method to append the contents of the first input string slice, followed by the second input string slice.</li>
  <li>Return the <code>result</code> string from the function.</li>
  <li>In the main function, create two String variables, <code>string1</code> and <code>string2</code>, and initialize them with appropriate values.</li>
  <li>Call the <code>concatenate_strings</code> function with references to <code>string1</code> and <code>string2</code> as arguments (using string slices). Store the result in a new variable called <code>concatenated_string</code>.</li>
  <li>Print the <code>concatenated_string</code> variable to the console.</li>
  <li>Compile and run the program to ensure it works as expected.</li>
</ol>

<h2>Checklist</h2>

<ol>
  <li>Write the <code>concatenate_strings</code> function signature.</li>
  <li>Implement the <code>concatenate_strings</code> function.</li>
  <li>Initialize two String variables in the main function.</li>
  <li>Call the <code>concatenate_strings</code> function with string slices of the variables.</li>
  <li>Print the result to the console.</li>
  <li>Compile and run the program to test its functionality.</li>
</ol>

</body>
</html>
