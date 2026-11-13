package org.accessware.prismer.example;

import android.app.Activity;
import android.os.Bundle;

public final class MainActivity extends Activity {
	static {
		// prism's JNI_OnLoad finds prism's Java classes through the class loader of the class that
		// loads it, so a class of the app's loads prism itself, before the Rust library linking it.
		System.loadLibrary("prism");
		System.loadLibrary("prismer_android_example");
	}

	private static native void speak();

	@Override
	protected void onCreate(Bundle savedInstanceState) {
		super.onCreate(savedInstanceState);
		// Starting Android's speech engine waits for a callback that arrives on this thread, so
		// prism runs on another.
		new Thread(MainActivity::speak).start();
	}
}
