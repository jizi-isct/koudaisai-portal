import {createContext, ReactNode, useCallback, useContext, useMemo, useState} from "react";
import {apiQueryClientType, DocumentCategoryRead, DocumentRead} from "@/lib";

interface ReadDocumentContextType {
  documents: Array<{ category: DocumentCategoryRead | null, documents: DocumentRead[] }> | undefined;
  isLoading: boolean;
  error: Error | null;
  refreshDocuments: () => Promise<void>;
}

const ReadDocumentContext = createContext<ReadDocumentContextType>({
  documents: undefined,
  isLoading: false,
  error: null,
  refreshDocuments: async () => {
    return
  },
});

export const useReadDocumentContext = () => useContext(ReadDocumentContext);

type ReadDocumentProviderProps = {
  children: ReactNode,
  queryClient: apiQueryClientType
}

export function ReadDocumentProvider({children, queryClient}: ReadDocumentProviderProps) {
  // state
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  // React Query
  const {data: documents, refetch} = queryClient.useQuery("get", "/documents/by-category");

  // Callbacks
  const refreshDocuments = useCallback(async () => {
    setIsLoading(true);
    try {
      await refetch();
    } catch (err) {
      setError(err instanceof Error ? err : new Error(String(err)));
    } finally {
      setIsLoading(false);
    }
  }, [refetch]);

  const contextValue = useMemo(() => ({
    documents,
    isLoading,
    error,
    refreshDocuments,
  }), [
    documents,
    isLoading,
    error,
    refreshDocuments
  ]);

  return (
    <ReadDocumentContext.Provider value={contextValue}>
      {children}
    </ReadDocumentContext.Provider>
  );
}