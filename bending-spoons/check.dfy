method f(n: int) returns (c: int)
  ensures n % 2 == 0 ==> c % 2 == 0
  ensures n > 10 ==> c > 0
{
  var k := n;
  c := 0;

  while (k >= 0)
    invariant n % 2 == 0 ==> k % 2 == 0
    invariant k % 2 == 0 ==> c % 2 == 0
    invariant k > 10 ==> c > 0
  {
    k := k - 2;
    c := c + k;
  }
}