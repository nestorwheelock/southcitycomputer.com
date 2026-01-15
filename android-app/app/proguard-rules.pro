# Keep WebView JavaScript interface
-keepclassmembers class * {
    @android.webkit.JavascriptInterface <methods>;
}

# Keep application class
-keep class com.southcitycomputer.app.** { *; }
