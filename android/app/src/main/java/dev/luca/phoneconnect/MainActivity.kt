package dev.luca.phoneconnect

import android.app.Activity
import android.os.Bundle
import android.widget.TextView

// Nessuna dipendenza AndroidX per ora: meno codice caricato, APK più piccolo.
class MainActivity : Activity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(TextView(this).apply {
            text = getString(R.string.app_name)
            textSize = 24f
            setPadding(48, 48, 48, 48)
        })
    }
}
