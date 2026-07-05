using System;
namespace Tests
{
	// Simple/basic login demonstration for tests
	public static class BasicLogin
	{
		// Returns true if username/password match hardcoded credentials
		public static bool Login(string username, string password)
		{
			const string user = "basit";
			const string pass = "password123";
			return string.Equals(username, user, StringComparison.Ordinal) &&
				   string.Equals(password, pass, StringComparison.Ordinal);
		}

		// Quick console demo when run directly
		public static void Main()
		{
			Console.Write("Username: ");
			var u = Console.ReadLine();
			Console.Write("Password: ");
			var p = Console.ReadLine();
			Console.WriteLine(Login(u, p) ? "Login successful" : "Login failed");
		}
	}
}
