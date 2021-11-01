using System;
using System.Threading;
using System.Threading.Tasks;
using System.IO;
using Discord;
using Discord.WebSocket;

namespace GlacBot
{
    class Program
    {
        public static DiscordSocketClient client;

		public async Task MainAsync() {
			Console.WriteLine("Starting GlacBot...");
            client = new DiscordSocketClient();

            await client.LoginAsync(TokenType.Bot, File.ReadAllText("token.txt"));
            await client.StartAsync();

            client.MessageReceived += MessageReceived;
            Console.WriteLine("GlacBot started!");
            Console.WriteLine("Press enter to kill the program");
            Console.ReadLine();

            foreach(string file in Directory.GetFiles(Environment.CurrentDirectory)){
                if(file.EndsWith(".png")){
                    File.Delete(file);
                    Console.WriteLine("Deleted "+file);
                }
            }

            await client.LogoutAsync();
            client.Dispose();
		}
        static void Main(string[] args) => new Program().MainAsync().GetAwaiter().GetResult();

        static string command = "glacify";
        public static async Task MessageReceived(SocketMessage message)
        {
            if(!message.Author.IsBot){
                if(message.Content.ToLower().StartsWith(command)){
                    if(message.Attachments.Count > 0){
                        foreach(var attachment in message.Attachments){
                            if(attachment.Filename.Contains(".png")){
                                await message.Channel.SendMessageAsync("glacifying...");
                                string file = await Utils.downloadFile(attachment.Url);
                                Utils.applyGlacses(file);
                                await message.Channel.SendFileAsync(file.Replace(".png","")+"-modified.png");
                                return;
                            }
                            else{
                                await message.Channel.SendMessageAsync("Only .PNG files can be glacced");
                                return;
                            }
                        }
                    }
                    try{
                        string username = message.Content.Substring(command.Length+1);
                        if (username.Length > 0)
                        {
                            await message.Channel.SendMessageAsync("glacifying "+username+"...");
                            await message.Channel.SendMessageAsync("Got uuid: "+await Utils.nameToUUID(username));
                            string fileName = await Utils.getSkinFile(username);
                            Utils.applyGlacses(fileName);
                            await message.Channel.SendFileAsync(fileName.Replace(".png","")+"-modified.png");
                        }
                    } catch(Exception ex) {
                        await message.Channel.SendMessageAsync("Invalid command usage or an error occoured");
                        Console.WriteLine(ex.Message);
                        Console.WriteLine(ex.StackTrace);
                    }
                }
            }
        }
    }
}
