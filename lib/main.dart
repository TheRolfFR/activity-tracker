import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter/foundation.dart';
import 'package:rinf/rinf.dart';
import 'package:window_manager/window_manager.dart';
import 'dart:ui' as ui;

import 'src/bindings/bindings.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await initializeRust(assignRustSignal);

  await windowManager.ensureInitialized();

  const options = WindowOptions(titleBarStyle: TitleBarStyle.hidden);

  windowManager.waitUntilReadyToShow(options, () async {
    await windowManager.setAsFrameless();
    await windowManager.show();
    await windowManager.focus();
  });

  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return FluentApp(
      title: 'Flutter Demo',
      theme: FluentThemeData(
        brightness: Brightness.dark,
        accentColor: Colors.orange,
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  // This widget is the home page of your application. It is stateful, meaning
  // that it has a State object (defined below) that contains fields that affect
  // how it looks.

  // This class is the configuration for the state. It holds the values (in this
  // case the title) provided by the parent (in this case the App widget) and
  // used by the build method of the State. Fields in a Widget subclass are
  // always marked "final".

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  @override
  Widget build(BuildContext context) {
    // This method is rerun every time setState is called, for instance as done
    // by the _incrementCounter method above.
    //
    // The Flutter framework has been optimized to make rerunning build methods
    // fast, so that you can just rebuild anything that needs updating rather
    // than having to individually change instances of widgets.
    return Container(
      height: MediaQuery.of(context).size.height * 0.7,
      width: MediaQuery.of(context).size.width * 0.9,
      // Below is the code for Linear Gradient.
      decoration: const BoxDecoration(
        gradient: LinearGradient(
          colors: [ui.Color(0xFFff0058), ui.Color(0xFFffbc00)],
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
        ),
      ),
      child: Row(
        // Left header divider
        children: [
          Container(
            // column
            decoration: const BoxDecoration(color: Colors.black),
            padding: const EdgeInsets.all(12),
            child: Column(
              children: [
                Row(
                  spacing: 10,
                  children: [
                    Image.asset("images/icon.png", width: 24),
                    Text(
                      "Activity\ntracker",
                      style: TextStyle(
                        fontSize: 14,
                        fontWeight: ui.FontWeight.bold,
                      ),
                    ),
                  ],
                ),
                _dayStat("Monday", "7h32"),
                _dayStat("Tuesday", "8h15"),
                _dayStat("Wednesday", "7h32"),
                _dayStat("Thursday", "8h15"),
                _dayStat("Friday", "7h32"),
              ],
            ),
          ),
          DragToMoveArea(child: Column(children: [WindowButtons()])),
        ],
      ),
    );
  }
}

class WindowButtons extends StatelessWidget {
  const WindowButtons({super.key});

  @override
  Widget build(final BuildContext context) {
    final theme = FluentTheme.of(context);

    switch (defaultTargetPlatform) {
      case TargetPlatform.windows:
      case TargetPlatform.macOS:
      case TargetPlatform.linux:
        return SizedBox(
          width: 138,
          height: 50,
          child: WindowCaption(
            brightness: theme.brightness,
            backgroundColor: Colors.transparent,
          ),
        );
      default:
        return const SizedBox.shrink();
    }
  }
}

Container _dayStat(String title, String duration) {
  return Container(
    child: Column(
      children: [
        Text(
          title,
          style: const TextStyle(fontWeight: FontWeight.w500, fontSize: 12),
        ),

        DecoratedBox(
            decoration: BoxDecoration(
                color: Color.fromARGB((4.19/100*255).round(), 255, 255, 255),
                borderRadius: BorderRadius.circular(8),
            ),
            child: SizedBox(
                width: 75,
                child: Container(
                    margin: const EdgeInsets.symmetric(vertical: 10),
                    child: Center(child: Text(duration, style: const TextStyle(
                    fontWeight:  FontWeight.bold,
                    fontSize: 24,
                    height: 1.0
                )))),
            )
        ),
      ],
    )
  );
}
