using Grpc.Core;
using Grpc.Net.Client;
using Helloworld; // This is the namespace of the generated classes

public class Program
{
    const string AUTH_TOKEN = "S3CR37-70K3N-2";

    public static async Task Main(string[] args)
    {
        var credentials = CallCredentials.FromInterceptor(async (context, metadata) =>
        {
            var token = AUTH_TOKEN; // await tokenProvider.GetTokenAsync(context.CancellationToken);
            metadata.Add("authorization", $"Bearer {token}");
        });

        // Create a channel
        var channel = GrpcChannel.ForAddress("http://[::1]:50051", new GrpcChannelOptions
        {
            Credentials = ChannelCredentials.Create(new SslCredentials(), credentials),
            UnsafeUseInsecureChannelCallCredentials = true
        });
        // TODO : Following code to use secure channel (https)
        // But System.Security.Authentication.AuthenticationException: Cannot determine the frame size or a corrupted frame was received.
        // var channel = GrpcChannel.ForAddress("https://[::1]:5001", new GrpcChannelOptions
        // {
        //     Credentials = ChannelCredentials.Create(new SslCredentials(), credentials)
        // });

        // Use the channel to create a client
        var greeterClient = new Greeter.GreeterClient(channel);
        var loverClient = new Lover.LoverClient(channel);

        // Ask for name
        Console.WriteLine("Who are you?");
        var myName = Console.ReadLine();

        var helloResponse = await greeterClient.SayHelloAsync(new HelloRequest { Name = myName });
        Console.WriteLine(helloResponse.Message);

        // Ask for lover's name
        Console.WriteLine("Who's your lover?");
        var loverName = Console.ReadLine();

        var loveResponse = await loverClient.SayLoveAsync(new LoveRequest { Name = loverName });
        Console.WriteLine(loveResponse.Message);
    }
}
