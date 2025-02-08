import datetime

# no args
datetime.datetime.now()

# wrong keywords
datetime.datetime.now(bad=datetime.timezone.utc)

# none args
datetime.datetime.now(None)

# none keywords
datetime.datetime.now(tz=None)

from datetime import datetime

# no args unqualified
datetime.now()

# uses `astimezone` method
datetime.now().astimezone()

# uses method chaining with replace and astimezone
datetime.datetime.now().replace(microsecond=0).astimezone()

# uses method chaining with multiple replaces and astimezone
datetime.datetime.now().replace(microsecond=0).replace(second=0).astimezone()
