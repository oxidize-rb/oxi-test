package oxi.test;

import org.jruby.*;
import org.jruby.runtime.load.BasicLibraryService;

public class OxiTestService implements BasicLibraryService {
    public static void systemLoad(String libPath) {
        System.load(libPath);
    }

    @Override
    public boolean basicLoad(final Ruby ruby) {
        RubyModule oxi_module = ruby.defineModule("Oxi");
        RubyModule oxi_test_module = ruby.defineModuleUnder("Test", oxi_module);
        oxi_test_module.defineAnnotatedMethods(OxiTest.class);
        return true;
    }
}
