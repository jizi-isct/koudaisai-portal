import {createContext, ReactNode, useCallback, useContext, useMemo} from "react";
import {
  apiQueryClientType,
  DocumentCategoryCreate,
  DocumentCategoryRead,
  DocumentCategoryUpdate,
  DocumentCreate,
  DocumentRead,
  DocumentUpdate
} from "@/lib";

interface WriteDocumentContextType {
  documents: Array<{ category: DocumentCategoryRead | null, documents: DocumentRead[] }> | undefined;
  categories: DocumentCategoryRead[];
  isLoading: boolean;
  fetchError: null | undefined;
  createDocument: (document: DocumentCreate) => Promise<void>;
  updateDocument: (documentId: string, document: DocumentUpdate) => Promise<void>;
  deleteDocument: (documentId: string) => Promise<void>;
  refetch: () => Promise<void>;
  createDocumentCategory: (document: DocumentCategoryCreate) => Promise<void>;
  updateDocumentCategory: (documentCategoryId: string, documentCategory: DocumentCategoryUpdate) => Promise<void>;
  deleteDocumentCategory: (documentCategoryId: string) => Promise<void>;
}

const WriteDocumentContext = createContext<WriteDocumentContextType>({
  documents: undefined,
  categories: [],
  isLoading: false,
  fetchError: null,
  refetch: async () => {
    return
  },
  createDocument: async () => {
    return
  },
  updateDocument: async () => {
    return
  },
  deleteDocument: async () => {
    return
  },
  createDocumentCategory: async () => {
    return
  },
  updateDocumentCategory: async () => {
    return
  },
  deleteDocumentCategory: async () => {
    return
  }
});

export const useWriteDocumentContext = () => useContext(WriteDocumentContext);

type WriteDocumentProviderProps = {
  children: ReactNode,
  queryClient: apiQueryClientType
}

export function WriteDocumentProvider({children, queryClient}: WriteDocumentProviderProps) {
  // React Query
  const {
    data: documents,
    refetch,
    error: fetchError
  } = queryClient.useQuery("get", "/documents/by-category", {params: {query: {include_empty_categories: true}}});
  const {mutateAsync: mutateCreateDocument} = queryClient.useMutation("post", "/documents")
  const {mutateAsync: mutateUpdateDocument} = queryClient.useMutation("patch", "/documents/{document_id}")
  const {mutateAsync: mutateDeleteDocument} = queryClient.useMutation("delete", "/documents/{document_id}")
  const {mutateAsync: mutateCreateDocumentCategory} = queryClient.useMutation("post", "/document-categories")
  const {mutateAsync: mutateUpdateDocumentCategory} = queryClient.useMutation("patch", "/document-categories/{category_id}")
  const {mutateAsync: mutateDeleteDocumentCategory} = queryClient.useMutation("delete", "/document-categories/{category_id}")

  const categories = useMemo(() => {
    return documents
      ?.map(({category}) => category)
      .filter((value) => value !== null) ?? [];
  }, [documents]);
  const isLoading = useMemo(() => documents === undefined, [documents]);

  const refetch_ = useCallback(async () => {
    await refetch()
  }, [refetch])

  // Callbacks
  const createDocument = useCallback(async (document: DocumentCreate) => {
    await mutateCreateDocument({body: document});
  }, [mutateCreateDocument]);

  const updateDocument = useCallback(async (documentId: string, document: DocumentUpdate) => {
    await mutateUpdateDocument({
      body: document,
      params: {
        path: {
          document_id: documentId
        }
      }
    });
  }, [mutateUpdateDocument]);

  const deleteDocument = useCallback(async (documentId: string) => {
    await mutateDeleteDocument({
      params: {
        path: {
          document_id: documentId
        }
      }
    });
  }, [mutateDeleteDocument]);

  const createDocumentCategory = useCallback(async (documentCategory: DocumentCategoryCreate) => {
    await mutateCreateDocumentCategory({body: documentCategory});
  }, [mutateCreateDocumentCategory]);

  const updateDocumentCategory = useCallback(async (documentCategoryId: string, documentCategory: DocumentCategoryUpdate) => {
    await mutateUpdateDocumentCategory({
      body: documentCategory,
      params: {
        path: {
          category_id: documentCategoryId
        }
      }
    });
  }, [mutateUpdateDocumentCategory]);

  const deleteDocumentCategory = useCallback(async (documentCategoryId: string) => {
    await mutateDeleteDocumentCategory({
      params: {
        path: {
          category_id: documentCategoryId
        }
      }
    });
  }, [mutateDeleteDocumentCategory]);

  const contextValue = useMemo(() => ({
    documents,
    categories,
    isLoading,
    refetch: refetch_,
    fetchError,
    createDocument,
    updateDocument,
    deleteDocument,
    createDocumentCategory,
    updateDocumentCategory,
    deleteDocumentCategory
  }), [
    documents,
    categories,
    isLoading,
    refetch_,
    fetchError,
    createDocument,
    updateDocument,
    deleteDocument,
    createDocumentCategory,
    updateDocumentCategory,
    deleteDocumentCategory
  ]);

  return (
    <WriteDocumentContext.Provider value={contextValue}>
      {children}
    </WriteDocumentContext.Provider>
  );
}