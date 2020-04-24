import java.nio.ByteBuffer;
import java.io.FileInputStream;

public class App {
  static {
    System.loadLibrary("wasm_bootstrap");
  }

  /** 
    @arg program a directly mapped Byte buffer containing the wasm program to execute
    @arg inputData a directly mapped Byte buffer containing the inputData to the program
  **/
  private native int reduce(ByteBuffer program, int programLength, ByteBuffer inputData);


  public static void main(String args[]) throws Exception {
    ByteBuffer buf = ByteBuffer.allocateDirect(80000);
    FileInputStream in = new FileInputStream("foo.wasm");

    int len = in.getChannel().read(buf);

    System.out.println(len);

    ByteBuffer p = ByteBuffer.allocateDirect(4086);
    in = new FileInputStream("a.csv");
    in.getChannel().read(p);



    System.out.println(new App().reduce(buf, len, p));
  }
}
