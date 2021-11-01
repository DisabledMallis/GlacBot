using System;
using System.Net.Http;
using System.Net;
using System.Threading.Tasks;
using System.IO;
using System.Drawing;
using System.Drawing.Imaging;
using System.Text.RegularExpressions;
using Newtonsoft.Json;

namespace GlacBot
{
    public class Utils
    {
        private static WebClient web_client = new WebClient();
        public static async Task<string> nameToUUID(string name)
        {
            PlayerDbRequest parsed = await getPlayerInfo(name);
            return parsed.Data.Player.Id;
        }
        public static async Task<string> getSkinFile(string name)
        {
            string uuid = await nameToUUID(name);
            string filename = name+".png";
            web_client.DownloadFile("https://crafatar.com/skins/"+uuid, filename);
            return filename;

        }
        public static async Task<string> downloadFile(string url)
        {
            string filename = Regex.Match(url,@"\/([A-Za-z0-9\-._~:?#\[\]@!$%&'()*+,;=]*).png").Groups[1].Value;
            filename += ".png";
            web_client.DownloadFile(url, filename);
            return filename;
        }
        public static async Task<PlayerDbRequest> getPlayerInfo(string name)
        {
            string playerData = web_client.DownloadString("https://playerdb.co/api/player/minecraft/"+name);
            return JsonConvert.DeserializeObject<PlayerDbRequest>(playerData);
        }
        public static void applyGlacses(string filename)
        {
            using(Image source = Image.FromFile(filename)){
                using(Graphics g = Graphics.FromImage(source)){
                    Pen p = new Pen(Brushes.Black, 1);
                    //draw glacses
                    g.DrawLine(p, new Point(37,11), new Point(50,11));
                    g.DrawLine(p, new Point(41,12), new Point(42,12));
                    g.DrawLine(p, new Point(45,12), new Point(46,12));
                }
                source.Save(filename.Replace(".png","")+"-modified.png");
            }
        }
    }
}