using System;

namespace CountOfCharacters
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.Write("Введите строку: ");
            string source = Console.ReadLine();

            Console.Write("Введите символ для поиска: ");
            char symbol = (char)(Console.Read());

            char[] arr = source.ToCharArray();
            int length = arr.Length;

            int count = 0;

            for(int i = 0; i < length; ++i)
            {
                if(arr[i] == symbol)
                {
                    count += 1;
                }
            }
            Console.WriteLine("Данный символ встречается в строке {0} раз", count);
        }
    }
}
