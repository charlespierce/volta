initSidebarItems({"fn":[["accepts_byte_ranges",""],["content_length","Determines the length of an HTTP response's content in bytes, using the HTTP `\"Content-Length\"` header."],["fetch_isize","Fetches just the `isize` field (the field that indicates the uncompressed size) of a gzip file from a URL. This makes two round-trips to the server but avoids downloading the entire gzip file. For very small files it's unlikely to be more efficient than simply downloading the entire file up front."],["fetch_uncompressed_size","Determines the uncompressed size of a gzip file hosted at the specified URL by fetching just the metadata associated with the file. This makes an extra round-trip to the server, so it's only more efficient than just downloading the file if the file is large enough that downloading it is slower than the extra round trips."],["load_isize","Loads the `isize` field (the field that indicates the uncompressed size) of a gzip file from disk."],["load_uncompressed_size","Determines the uncompressed size of the specified gzip file on disk."],["unpack_isize","Unpacks the `isize` field from a gzip payload as a 64-bit integer."]],"struct":[["Tarball","A Node installation tarball."]]});