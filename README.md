# Instructions

To run this program you first need to go to your steam library, right click on tf2 and select "Properties..."

![](assets/20220804_033754_image.png)

And add to launch parameters `-condebug -conclearlog -usercon`

![](assets/20220804_033959_image.png)

Next, you'll need to modify your autoexec.cfg inside tf2rootfolder/tf/cfg/ to add these lines:

```
ip 0.0.0.0
rcon_password "YOUR PASSWORD"
net_start
```

Then, you need to configure your config.json located in the executable's folder to target you or your friends or whoever you desire. Here's an example:

```json
{
    "rcon_password": "PASSWORD", // write your password set in autoexec.cfg
    "tf2_path": "... /steamapps/common/Team Fortress 2/", // your tf2 path
    "usernames": [
        "YOUR STEAM USERNAME",
        "YOUR FRIEND'S USERNAME" // delete this if you don't want to taunt when your friend kill
    ],
    "message": "" // message you want to send when any user in usernames kill someone
}
```

You may add extra settings to adjust your gameplay like:


| Key             | Value           | Description                                                                                                                                                                                                       |
| ----------------- | ----------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| rcon_password   | string          | Password set in autoexec.cfg                                                                                                                                                                                      |
| tf2_path        | string          | Path to tf2 root folder may be absolute or relative                                                                                                                                                               |
| usernames       | list of strings | When any of these usernames kills any enemy (or specified by username_victim) you'll taunt. You need to add yourself to this list if you want to taunt when killing                                               |
| message         | string          | Message you'll send when any user in usernames kill someone or username_victim                                                                                                                                    |
| port            | string          | Server's (and rcon's) port to connect to, default to "27015", DO NOT FORWARD THIS PORT.                                                                                                                           |
| username_victim | string          | You'll taunt only to this enemy if he was killed by any user in usernames, if this variable is not empty default to an empty string ("")                                                                          |
| use_spinbot     | boolean         | Uses +left to spin for one second after any user in usernames kill someone or username_victim, default to false, overrides taunting and you can customize its rotational speed with cl_yawspeed, default to false |
| use_soundpad    | boolean         | Only available in Windows, plays a sound when any user in usernames kill someone or username_victim, the sound used is in index 1, default to false                                                               |
| soundpad_path   | string          | Only available in Windows, Path to soundpad root folder, default to empty string                                                                                                                                  |

You'll need to put your favorite taunt to slot 1!

# Building

Use cargo to build and install dependencies

`cargo build --release`