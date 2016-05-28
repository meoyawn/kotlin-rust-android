package adeln.kotlinrustandroid

import android.app.Activity
import android.os.Bundle
import android.widget.LinearLayout
import android.widget.TextView

val libHello = System.loadLibrary("hello")
external fun test(): String

class MainActivity : Activity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)
    setContentView(LinearLayout(this).apply {
      orientation = LinearLayout.VERTICAL
      addView(TextView(context).apply {
        text = "this is kotlin"
      })
      addView(TextView(context).apply {
        text = test()
      })
    })
  }
}
