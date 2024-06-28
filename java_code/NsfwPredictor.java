import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.Socket;
import java.net.SocketException;
import java.net.UnknownHostException;

class NsfwPredictor {
    private static native String predict(String filename);

    static {
        System.loadLibrary("nsfw_lib");
    }

    public static void main(String[] args) {
        String[] results;
        System.out.println("Calling Rust!");

        // TIME CONSUMING TEST
        // ================= ( TEST ) ================= //
        var time = System.currentTimeMillis();;         //
        for (int i = 0; i < 100; i++) {                 //
            NsfwPredictor.predict(args[0]);             //
        }                                               //
        var time1 = System.currentTimeMillis();         //
        // ================= ( TEST ) ================= //
        
        System.out.println("Thanks, Rust! Average time of your execution has been " + ((time1 - time) / 100) + " millis");
    }
}
