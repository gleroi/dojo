-module(firsttry).

-export([one/2, start/0, two/1]).

-spec firsttry:one(Other::pid(), Counter::number()) -> nil().
one(Other, 0) ->
    monitor(process, Other),
    Other ! {one, 0, self()},
    one(Other, 1);
one(Other, Cnt) ->
    timer:sleep(250),
    receive
      {Name, Counter} ->
        io:format("one received from ~w : ~w ~n", [Name, Counter]),
        Other ! {one, Cnt, self()},
        one(Other, Cnt + 1);
      Msg -> 
          io:format("one received ~w ~n", [Msg]),
          io:format("one exited~n")
    end.

%% two waits for a message from one, then respond. 10 times.
two(10) -> io:format("two exited~n");
two(Cnt) ->
    timer:sleep(250),
    receive
      {Name, Counter, From} ->
        io:format("two received from ~w : ~w ~n",
                [Name, Counter]),
        From ! {two, Counter}
    end,
    two(Cnt + 1).

start() ->
    spawn(fun () ->
        process_flag(trap_exit, true),
        Pid2 = spawn(firsttry, two, [0]),
        spawn_link(firsttry, one, [Pid2, 0]),
        receive
            Msg -> io:format("start ~w~n", [Msg])
        end,
        io:format("start done~n")
    end).
