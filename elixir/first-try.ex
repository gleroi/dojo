defmodule FirstTry do

# one exchanges message with two and detect when two exits
  def one(other, cnt) do
    Process.monitor(other)
    send(other, {:one, cnt, self()})
    one_rec(other, cnt+1)
    IO.puts("one exited")
  end

  def one_rec(other, cnt) do
    Process.sleep(250)
    receive do
      {name, counter} -> 
        IO.puts("one receive: #{name} @ #{counter}")
        send(other, {:one, cnt, self()})
        one_rec(other, cnt+1)
      msg -> 
        IO.puts("one receive: #{inspect msg} EXIT!!!")
    end
  end

  def two(cnt) do
    Process.sleep(250)
    unless cnt >= 10 do
      receive do
        {name, counter, pid} -> 
          IO.puts("two receive: #{name} @ #{counter}")
          send pid, {"two", cnt}
      end
      two(cnt+1)
    end
  end

end

pid2 = spawn(FirstTry, :two, [0])
spawn(FirstTry, :one, [pid2, 0])



