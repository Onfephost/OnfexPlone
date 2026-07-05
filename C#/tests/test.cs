using System;
using System.Numerics;
enum Token
{
    Ident,
    Semi,
    Equal,
}
public class Lexer
    {
        public Vector<Token> lex(string txt)
        {
            for(int i = 0; i < txt.Length(); i++)
        {
            char ch = txt[i];
            Console.WriteLine(ch);
        }
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
