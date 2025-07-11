package com.example.chat_app

import android.content.Context
import android.os.Bundle
import android.widget.ArrayAdapter
import android.widget.Button
import android.widget.ListView
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

//private var count = 0
class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        val btn: Button = findViewById(R.id.touchBtn)

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
            //tests to check if i still know how to write android code after finishing uni
            //server testing
            //count++
            //txt.text = "You touched me $count times"
            CoroutineScope(Dispatchers.IO).launch{
                try{
                    val socket = Socket(ipAddress, port)
                    val pw = PrintWriter(socket.getOutputStream(), true)
                    pw.println()
                    //read server response
                    val br = socket.getInputStream().bufferedReader().readLine()
                    val responseJson = JSONObject(br)
                    val directories = responseJson.getJSONArray("directories")
                    val contents = responseJson.getJSONArray("last_dir_contents")
                    //save data from the server message (json) to a list
                    val listItems = mutableListOf<String>()
                    listItems.add("Directories:")
                    for (i in 0 until directories.length()) {
                        listItems.add(directories.getString(i))
                    }
                    listItems.add("Last Directory Contents:")
                    for (i in 0 until contents.length()) {
                        listItems.add(contents.getString(i))
                    }
                    withContext(Dispatchers.Main){
                        //display list in listview
                        val listView: ListView = findViewById(R.id.lvDir)
                        val adapter = ArrayAdapter(this@MainActivity, android.R.layout.simple_list_item_1, listItems)
                        listView.adapter = adapter
                    }
                    socket.close()
                } catch(e: Exception){
                    withContext(Dispatchers.Main){
                    }
                }
            }
        }

    }

}