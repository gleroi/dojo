defmodule FirstTry do

# one exchanges message with two and detect when two exits
  def one(other, 0) do
    Process.monitor(other)
    send(other, {:one, 0, self()})
    one(other, 1)
  end

  def one(other, cnt) do
    Process.sleep(250)
    receive do
      {name, counter} -> 
        IO.puts("one receive: #{name} @ #{counter}")
        send(other, {:one, cnt, self()})
        one(other, cnt+1)
      msg -> 
        IO.puts("one receive: #{inspect msg} EXIT!!!")
        IO.puts("one exited")
    end
  end

  def two(10) do
    IO.puts("two exited")
  end
  def two(cnt) do
    Process.sleep(250)
    receive do
      {name, counter, pid} -> 
        IO.puts("two receive: #{name} @ #{counter}")
        send pid, {"two", cnt}
    end
    two(cnt+1)
  end

end

pid2 = spawn(FirstTry, :two, [0])
spawn(FirstTry, :one, [pid2, 0])
