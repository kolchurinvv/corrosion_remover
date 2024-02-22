from python import Python 

fn main() raises:
	let chromadb = Python.import_module("chromadb")
	let chroma_client = chromadb.HttpClient(host='localhost', port=8000)
	let collection = chroma_client.create_collection(name="my_collection")
	collection.add(
		documents=["This is a document", "This is another document"],
		metadatas=[{"source": "my_source"}, {"source": "my_source"}],
		ids=["id1", "id2"]
		)
	let results = collection.query(
		query_texts=["This is a query document"],
		n_results=2
		)
	print(results)
