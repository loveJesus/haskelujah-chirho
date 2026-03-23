-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

-- | Built-in concurrency primitives. No package install needed.
module Haskelujah.Concurrent (
    -- Thread management
    fork,
    delay,
    -- Channels
    Chan,
    newChan,
    writeChan,
    readChan,
    -- MVars
    MVar,
    newMVar,
    readMVar,
    takeMVar,
    putMVar,
) where

-- | Fork a new thread (placeholder — will use FFI).
fork :: IO () -> IO ()
fork action = action -- sequential placeholder

-- | Delay for n microseconds (placeholder).
delay :: Int -> IO ()
delay _ = return ()

-- | A simple channel (placeholder).
data Chan a = Chan

-- | Create a new channel.
newChan :: IO (Chan a)
newChan = return Chan

-- | Write to a channel.
writeChan :: Chan a -> a -> IO ()
writeChan _ _ = return ()

-- | Read from a channel.
readChan :: Chan a -> IO a
readChan _ = error "readChan: not yet implemented"

-- | A mutable variable (placeholder).
data MVar a = MVar

newMVar :: a -> IO (MVar a)
newMVar _ = return MVar

readMVar :: MVar a -> IO a
readMVar _ = error "readMVar: not yet implemented"

takeMVar :: MVar a -> IO a
takeMVar _ = error "takeMVar: not yet implemented"

putMVar :: MVar a -> a -> IO ()
putMVar _ _ = return ()
