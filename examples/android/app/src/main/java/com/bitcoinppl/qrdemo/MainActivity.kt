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
import androidx.compose.ui.tooling.preview.Preview
import com.bitcoinppl.qrdemo.ui.theme.QrDemoTheme
import androidx.compose.foundation.layout.*
import androidx.compose.ui.unit.dp
import com.bbqr.Bbqr
import com.bbqr.ContinuousJoiner
import com.bbqr.Encoding
import com.bbqr.FileType
import com.bbqr.Split
import com.bbqr.SplitOptions
import com.bbqr.Version

fun largeString(): String {
    return "bacon".repeat(25)
}

fun split(): List<String> {
    val large = largeString().toByteArray()
    val options = SplitOptions(Encoding.HEX, Version.V01, Version.V02)
    val split = Split.tryFromData(large, FileType.UNICODE_TEXT, options)
    return split.parts()
}

fun continuousJoiner(parts: List<String>): String {
    val continuousJoiner = ContinuousJoiner()
    for (part in parts) {
        when (val result = continuousJoiner.addPart(part)) {
            is ContinuousJoiner.Result.NotStarted -> println("not started")
            is ContinuousJoiner.Result.InProgress -> println("added item, ${result.partsLeft} parts left")
            is ContinuousJoiner.Result.Complete -> return String(result.joined.data())
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
        Divider(modifier = Modifier.padding(vertical = 30.dp))
        Text(text = "joined", modifier = Modifier.padding(bottom = 10.dp))
        Text(text = continousJoiner(split()))
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

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    QrDemoTheme {
        Greeting("Android")
    }
}