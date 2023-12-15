package oxi.test;

import org.jruby.*;
import org.jruby.anno.JRubyMethod;
import org.jruby.anno.JRubyModule;
import org.jruby.runtime.ThreadContext;
import org.jruby.runtime.builtin.IRubyObject;

@SuppressWarnings("serial")
@JRubyModule(name = "Xml")
public class OxiTest {

//    private static native String buildNative(String input);

    //    https://github.com/jruby/jruby/wiki/JRubyMethod_Signatures
    @JRubyMethod(name = "hello", module = true, meta = true)
    public static IRubyObject hello(ThreadContext context, IRubyObject self, RubyString name) {
        return RubyString.newString(context.getRuntime(), "Hello, " + name.asJavaString());
    }
}