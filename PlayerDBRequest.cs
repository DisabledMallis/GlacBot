namespace GlacBot
{
    using System;
    using System.Collections.Generic;

    using System.Globalization;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Converters;

    public partial class PlayerDbRequest
    {
        [JsonProperty("code")]
        public string Code { get; set; }

        [JsonProperty("message")]
        public string Message { get; set; }

        [JsonProperty("data")]
        public Data Data { get; set; }

        [JsonProperty("success")]
        public bool Success { get; set; }
    }

    public partial class Data
    {
        [JsonProperty("player")]
        public Player Player { get; set; }
    }

    public partial class Player
    {
        [JsonProperty("meta")]
        public Meta Meta { get; set; }

        [JsonProperty("username")]
        public string Username { get; set; }

        [JsonProperty("id")]
        public string Id { get; set; }

        [JsonProperty("raw_id")]
        public string RawId { get; set; }

        [JsonProperty("avatar")]
        public Uri Avatar { get; set; }
    }

    public partial class Meta
    {
        [JsonProperty("name_history")]
        public List<NameHistory> NameHistory { get; set; }
    }

    public partial class NameHistory
    {
        [JsonProperty("name")]
        public string Name { get; set; }

        [JsonProperty("changedToAt", NullValueHandling = NullValueHandling.Ignore)]
        public long? ChangedToAt { get; set; }
    }
}
