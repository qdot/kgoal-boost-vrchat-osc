# KGoal Boost VRChat OSC Input Utility

Connects to and takes pressure readings from the [KGoal
Boost](https://www.kgoal.com/?rfsn=6183821.98f400&utm_source=refersion&utm_medium=affiliate&utm_campaign=6183821.98f400)
and routes it to the /avatar/parameters/Squeeze OSC endpoint.

Reads come in through a BLE Notify Characteristic, events for which are streamed in asynchronously, converted to OSC, and send via UDP to the localhost 9000 port that VRChat listens on.

Mostly a [btleplug](https://github.com/deviceplug/btleplug) example munged with the smallest amount of tokio and rosc possible to get things out.