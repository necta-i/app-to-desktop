package com.example.chat_app

import android.content.Context
import android.os.Bundle
import android.widget.Button
import android.widget.TextView
import androidx.activity.enableEdgeToEdge
import androidx.appcompat.app.AppCompatActivity
import androidx.core.view.ViewCompat
import androidx.core.view.WindowInsetsCompat
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import org.json.JSONObject
import java.io.PrintWriter
import java.net.Socket

private var count = 0
class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        val btn: Button = findViewById(R.id.touchBtn)
        val txt: TextView = findViewById(R.id.textView)

        fun readJson(context: Context, fileName: String): String{
            return context.assets.open(fileName).bufferedReader().use{
                it.readText()
            }
        }

        val jsonString = readJson(this, "targetFile.json")
        val jsonObj = JSONObject(jsonString)
        val ipAddress = jsonObj.getString("address")
        val port = jsonObj.getInt("port")
        btn.setOnClickListener {
            count++
            txt.text = "You touched me $count times"
            CoroutineScope(Dispatchers.IO).launch{
                try{
                    val socket = Socket(ipAddress, port)
                    val pw = PrintWriter(socket.getOutputStream(), true)
                    pw.println(txt.text)
                    pw.println()
                    val br = socket.getInputStream().bufferedReader().readLine()
                    withContext(Dispatchers.Main){
                        txt.text = br
                    }
                    socket.close()
                } catch(e: Exception){
                    withContext(Dispatchers.Main){
                        txt.text = "Woopsie, the server did a fucky wucky ${e.message}"
                    }
                }
            }
        }

    }

}