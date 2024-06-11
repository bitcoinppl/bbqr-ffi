package com.bitcoinppl.qrdemo

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.bitcoinppl.qrdemo.ui.theme.QrDemoTheme
import androidx.compose.foundation.layout.*
import androidx.compose.ui.unit.dp
import org.bitcoinppl.bbqr.ContinuousJoiner
import org.bitcoinppl.bbqr.Encoding
import org.bitcoinppl.bbqr.FileType
import org.bitcoinppl.bbqr.Split
import org.bitcoinppl.bbqr.SplitOptions
import org.bitcoinppl.bbqr.Version
import org.bitcoinppl.bbqr.ContinuousJoinResult

fun largeString(): String {
    return "bacon".repeat(25)
}

fun split(): List<String> {
    val large = largeString().toByteArray()
    // val defaultOptions = defaultSplitOptions()
    val options = SplitOptions(encoding = Encoding.HEX, minVersion = Version.V01, maxVersion = Version.V02)
    val split = Split.tryFromData(large, FileType.UNICODE_TEXT, options)
    return split.parts()
}

fun continuousJoiner(parts: List<String>): String {
    val continuousJoiner = ContinuousJoiner()
    for (part in parts) {
        when (val result = continuousJoiner.addPart(part)) {
            is ContinuousJoinResult.NotStarted -> println("not started")
            is ContinuousJoinResult.InProgress -> println("added item, ${result.partsLeft} parts left")
            is ContinuousJoinResult.Complete -> return String(result.joined.data())
        }
    }
    return ""
}

@Composable
fun ContentView() {
    Column(modifier = Modifier.padding(16.dp)) {
        Text(text = "split", modifier = Modifier.padding(bottom = 10.dp))
        split().forEach { part ->
            Text(text = part, modifier = Modifier.padding(2.dp))
        }
        Text(text = "joined", modifier = Modifier.padding(bottom = 10.dp))
        Text(text = continuousJoiner(parts = split()))
    }
}


class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            QrDemoTheme {
                Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                    ContentView()
                }
            }
        }
    }
}
