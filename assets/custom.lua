function OnKillCallback(username, username_victim) -- username_victim is the one killed by username
    if username == "niidea" and username_victim == "pepe" then
        tf2:taunt(1) -- to taunt in game. The argument is the slot of the taunt to be used.
        
        tf2:say(username_victim .. ", get dunked on kid") -- to say whatever you want in tf2's chat

        return true -- use 'return true' to let the taunter know this username and username_victim are your ideal victims and want them to show in the logs(?
    end

    return false -- use 'return false' to let the taunter know this username and username_victim are *NOT* your ideal victims and *DON'T* want them to show in the logs(?
end