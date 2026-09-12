# Desktop_shell
This is the official repository for the Noctura Laptop/Desktop devices

# Important Notes
For any contributors, I would like to mention some things about this project, and some problems i faces
and the things i did to solve them. Here are a list of them in order

Issues fixed
- How to make icons work

## How to make icons work

The issue is, you are not allowed to directly load "/var/lib/flatpak...." in the front end. But, you are allowed to load an immage,
This means,



```
Backend {                                           Frontend {
    "/var/lib/flatpak"            ===X==>                   <img src="/var/lib/flatpak" />
}                                                   }
```
Will NOT work

So the process looks a bit more like:-
```
Backend {                                                                                             Frontend  {
    "/var/lib/flatpak"   =>  Turn into a URL  => "app-icons://localhost.."        ======>                 <img src="URL" />
}  
```                                                                                                   }