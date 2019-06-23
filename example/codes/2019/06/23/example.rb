require 'redis'
require 'logger'
require 'securerandom'

INTERVAL = 0.3

SENTINELS = [
  { host: '127.0.0.1', port: 5000 },
  { host: '127.0.0.1', port: 5001 },
  { host: '127.0.0.1', port: 5002 },
]

redis = Redis.new(
  url: 'redis://mymaster',
  sentinels: SENTINELS,
  role: :master,
)

logger = Logger.new(STDOUT)

loop do
  begin
    key = SecureRandom.uuid

    redis.set(key, "value: #{key}")
    logger.info("Set: #{key}")
    sleep 0.3
  rescue => e
    logger.error(e)
  end
end
