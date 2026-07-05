using System;

public class Hesap
    {
        public int Topla(int a, int b)
        {
            return a + b;
        }
    }
class Program
{
    static void Main()
    {
        Hesap hesap = new Hesap();
		
        Console.WriteLine(hesap.Topla(5, 8));
    }
}
